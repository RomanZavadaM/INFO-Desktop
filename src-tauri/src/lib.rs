
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use serde::{Deserialize, Serialize};
use tauri::command;

use info_desktop_core::registration;
use info_desktop_core::transport;
use info_desktop_core::{paths, profile, session};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, State};
use transport::{checked_url, client_with_cert, ReadonlyResponse};

const MAX_PREVIEW_BYTES: usize = 8 * 1024 * 1024;

#[derive(Serialize)]
struct ReadonlyBinaryResponse {
    status: u16,
    content_type: String,
    body_base64: String,
}

#[command]
fn import_registration_text(text: String) -> Result<registration::RegistrationSummary, String> {
    registration::import_registration(&text)
}


#[derive(Deserialize)]
struct EnrollmentResponse {
    device_id: String,
    device_secret: String,
    credential_expires_at: String,
}

#[derive(Serialize)]
struct ActivationResult {
    device_id: String,
    credential_expires_at: String,
    storage: String,
}

#[command]
async fn activate_registration(
    server: String,
    port: u16,
    registration_id: String,
    device_name: String,
    server_ca_der: String,
) -> Result<ActivationResult, String> {
    let base = format!("https://{}:{}", server.trim(), port);
    let url = checked_url(&(base + "/api/info-desktop-registration/v1/enroll"))?;
    let token = registration::registration_token(&registration_id)?;

    let payload = serde_json::json!({
        "registration_id": registration_id,
        "registration_token": token,
        "device_name": device_name.trim(),
    });

    let response = client_with_cert(&server_ca_der)?
        .request(reqwest::Method::POST, url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .body(payload.to_string())
        .send()
        .await
        .map_err(|error| error.to_string())?;

    let status = response.status().as_u16();
    let body = response.text().await.map_err(|error| error.to_string())?;
    if status < 200 || status >= 300 {
        return Err(format!("Registration failed: HTTP {status}"));
    }

    let enrolled: EnrollmentResponse =
        serde_json::from_str(&body).map_err(|_| "Invalid enrollment response".to_string())?;
    registration::store_device_credential(&enrolled.device_id, &enrolled.device_secret)?;
    registration::forget_registration_token(&registration_id)?;

    Ok(ActivationResult {
        device_id: enrolled.device_id,
        credential_expires_at: enrolled.credential_expires_at,
        storage: "os-secure-store".to_string(),
    })
}

#[command]
async fn device_get(
    url: String,
    device_id: String,
    server_ca_der: String,
) -> Result<ReadonlyResponse, String> {
    transport::device_get(&url, &device_id, &server_ca_der).await
}

#[command]
async fn device_post(
    url: String,
    device_id: String,
    server_ca_der: String,
    body: String,
) -> Result<ReadonlyResponse, String> {
    transport::device_post(&url, &device_id, &server_ca_der, body).await
}

#[command]
async fn device_get_preview(
    url: String,
    device_id: String,
    server_ca_der: String,
) -> Result<ReadonlyBinaryResponse, String> {
    let parsed = checked_url(&url)?;
    let secret = registration::device_secret(&device_id)?;
    let response = client_with_cert(&server_ca_der)?
        .get(parsed)
        .header(reqwest::header::ACCEPT, "image/*")
        .header("X-INFO-Device", device_id)
        .bearer_auth(secret)
        .send()
        .await
        .map_err(|error| error.to_string())?;
    let status = response.status().as_u16();
    let content_type = response.headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();
    if status < 200 || status >= 300 {
        return Ok(ReadonlyBinaryResponse { status, content_type, body_base64: String::new() });
    }
    if !content_type.to_ascii_lowercase().starts_with("image/") {
        return Err("Preview is not an image".to_string());
    }
    if let Some(length) = response.content_length() {
        if length as usize > MAX_PREVIEW_BYTES {
            return Err("Preview is larger than the 8 MiB safety limit".to_string());
        }
    }
    let bytes = response.bytes().await.map_err(|error| error.to_string())?;
    if bytes.len() > MAX_PREVIEW_BYTES {
        return Err("Preview is larger than the 8 MiB safety limit".to_string());
    }
    Ok(ReadonlyBinaryResponse {
        status,
        content_type,
        body_base64: BASE64.encode(bytes),
    })
}


#[derive(Deserialize)]
struct RotationResponse {
    device_id: String,
    device_secret: String,
    credential_expires_at: String,
}

#[derive(Serialize)]
struct RotationResult {
    device_id: String,
    credential_expires_at: String,
    storage: String,
}

#[command]
async fn rotate_device_credential(
    server: String,
    device_id: String,
    server_ca_der: String,
) -> Result<RotationResult, String> {
    let base = server.trim().trim_end_matches('/');
    let url = checked_url(&(base.to_string() + "/api/info-desktop-registration/v1/rotate"))?;
    let old_secret = registration::device_secret(&device_id)?;
    let payload = serde_json::json!({
        "device_id": device_id,
        "device_secret": old_secret,
    });

    let response = client_with_cert(&server_ca_der)?
        .request(reqwest::Method::POST, url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .body(payload.to_string())
        .send()
        .await
        .map_err(|error| error.to_string())?;

    let status = response.status().as_u16();
    let body = response.text().await.map_err(|error| error.to_string())?;
    if status < 200 || status >= 300 {
        return Err(format!("Credential rotation failed: HTTP {status}"));
    }

    let rotated: RotationResponse =
        serde_json::from_str(&body).map_err(|_| "Invalid rotation response".to_string())?;
    if rotated.device_id != device_id {
        return Err("Rotation response device mismatch".to_string());
    }
    registration::store_device_credential(&rotated.device_id, &rotated.device_secret)?;

    Ok(RotationResult {
        device_id: rotated.device_id,
        credential_expires_at: rotated.credential_expires_at,
        storage: "os-secure-store".to_string(),
    })
}

/// З'єднання, яким користується консоль через відкрите вікно (адреса, номер пристрою, довірений сертифікат; ключ береться зі сховища ОС).
struct Connection {
    base: String,
    device_id: String,
    server_ca_der: String,
}

#[derive(Default)]
struct ConsoleState {
    connection: Arc<Mutex<Option<Connection>>>,
    shared: Mutex<Option<Arc<session::Shared>>>,
    port: Mutex<u16>,
}

const LOCK_ERROR: &str = "Internal lock error";

/// Виконує запит консолі тим самим транспортом, що й вікно; кожну дію показує у вікні (подія `console-activity`).
fn console_perform(app: AppHandle, connection: Arc<Mutex<Option<Connection>>>) -> Arc<session::Perform> {
    Arc::new(move |method: &str, path: &str, body: &str| {
        let conn = match connection.lock() {
            Ok(guard) => guard.as_ref().map(|c| (c.base.clone(), c.device_id.clone(), c.server_ca_der.clone())),
            Err(_) => None,
        };
        let (status, text) = match conn {
            None => (599u16, "Window is not connected to a server".to_string()),
            Some((base, device_id, ca)) => {
                let url = format!("{}{}{}", base, profile::API, path);
                let result = if method == "POST" {
                    tauri::async_runtime::block_on(transport::device_post(&url, &device_id, &ca, body.to_string()))
                } else {
                    tauri::async_runtime::block_on(transport::device_get(&url, &device_id, &ca))
                };
                match result {
                    Ok(r) => (r.status, r.body),
                    Err(e) => (599u16, e),
                }
            }
        };
        let _ = app.emit("console-activity", serde_json::json!({"method": method, "path": path, "status": status}));
        (status, text)
    })
}

/// Вікно повідомляє нативному шару про з'єднання й дозвіл консолі. Профіль з'єднання (без секретів) завжди оновлюється для автономної
/// консолі; вхід для консолі через вікно відкривається лише коли дозволено (локальна адреса 127.0.0.1, пароль сеансу у файлі сеансу).
#[command]
fn console_bridge_sync(
    app: AppHandle,
    state: State<'_, ConsoleState>,
    enabled: bool,
    server: String,
    device_id: String,
    server_ca_der: String,
) -> Result<(), String> {
    let dir = paths::config_dir()?;
    let session_file = session::session_path(&dir);
    let mut shared_slot = state.shared.lock().map_err(|_| LOCK_ERROR.to_string())?;
    if server.trim().is_empty() || device_id.trim().is_empty() || server_ca_der.trim().is_empty() {
        *state.connection.lock().map_err(|_| LOCK_ERROR.to_string())? = None;
        if let Some(shared) = shared_slot.as_ref() {
            shared.enabled.store(false, Ordering::SeqCst);
        }
        session::remove_session(&session_file);
        return Ok(());
    }
    let prof = profile::Profile::from_server_url(&server, &device_id, &server_ca_der)?;
    profile::save_profile(&dir.join("connection.json"), &prof)?;
    *state.connection.lock().map_err(|_| LOCK_ERROR.to_string())? = Some(Connection {
        base: format!("https://{}:{}", prof.server, prof.port),
        device_id: prof.device_id.clone(),
        server_ca_der: prof.server_ca_der.clone(),
    });
    if !enabled {
        if let Some(shared) = shared_slot.as_ref() {
            shared.enabled.store(false, Ordering::SeqCst);
        }
        session::remove_session(&session_file);
        return Ok(());
    }
    if shared_slot.is_none() {
        let shared = Arc::new(session::Shared { token: session::new_token()?, enabled: AtomicBool::new(false) });
        let port = session::spawn_listener(shared.clone(), console_perform(app.clone(), state.connection.clone()))?;
        *state.port.lock().map_err(|_| LOCK_ERROR.to_string())? = port;
        *shared_slot = Some(shared);
    }
    let shared = shared_slot.as_ref().cloned().ok_or_else(|| LOCK_ERROR.to_string())?;
    shared.enabled.store(true, Ordering::SeqCst);
    let port = *state.port.lock().map_err(|_| LOCK_ERROR.to_string())?;
    session::write_session(&session_file, &session::SessionInfo { port, token: shared.token.clone() })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .manage(ConsoleState::default())
        .invoke_handler(tauri::generate_handler![import_registration_text, activate_registration, rotate_device_credential, device_get, device_post, device_get_preview, console_bridge_sync])
        .build(tauri::generate_context!())
        .expect("error while building INFO Desktop");
    app.run(|_app, event| {
        if let tauri::RunEvent::Exit = event {
            if let Ok(dir) = paths::config_dir() {
                session::remove_session(&session::session_path(&dir));
            }
        }
    });
}
