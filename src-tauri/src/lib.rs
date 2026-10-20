mod registration;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use serde::{Deserialize, Serialize};
use tauri::command;
use url::Url;

const MAX_PREVIEW_BYTES: usize = 8 * 1024 * 1024;

#[derive(Serialize)]
struct ReadonlyResponse {
    status: u16,
    body: String,
}

#[derive(Serialize)]
struct ReadonlyBinaryResponse {
    status: u16,
    content_type: String,
    body_base64: String,
}

fn checked_url(url: &str) -> Result<Url, String> {
    let parsed = Url::parse(url).map_err(|_| "Invalid server URL".to_string())?;
    if parsed.scheme() != "https" {
        return Err("INFO Desktop requires https:// server connections".to_string());
    }
    Ok(parsed)
}

fn client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .user_agent("INFO-Desktop/0.1-r1")
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|error| error.to_string())
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
) -> Result<ActivationResult, String> {
    let base = format!("https://{}:{}", server.trim(), port);
    let url = checked_url(&(base + "/api/info-desktop-registration/v1/enroll"))?;
    let token = registration::registration_token(&registration_id)?;

    let payload = serde_json::json!({
        "registration_id": registration_id,
        "registration_token": token,
        "device_name": device_name.trim(),
    });

    let response = client()?
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
) -> Result<ReadonlyResponse, String> {
    let parsed = checked_url(&url)?;
    let secret = registration::device_secret(&device_id)?;
    let response = client()?
        .get(parsed)
        .header(reqwest::header::ACCEPT, "application/json")
        .header("X-INFO-Device", device_id)
        .bearer_auth(secret)
        .send()
        .await
        .map_err(|error| error.to_string())?;
    let status = response.status().as_u16();
    let body = response.text().await.map_err(|error| error.to_string())?;
    Ok(ReadonlyResponse { status, body })
}

#[command]
async fn device_get_preview(
    url: String,
    device_id: String,
) -> Result<ReadonlyBinaryResponse, String> {
    let parsed = checked_url(&url)?;
    let secret = registration::device_secret(&device_id)?;
    let response = client()?
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
) -> Result<RotationResult, String> {
    let base = server.trim().trim_end_matches('/');
    let url = checked_url(&(base.to_string() + "/api/info-desktop-registration/v1/rotate"))?;
    let old_secret = registration::device_secret(&device_id)?;
    let payload = serde_json::json!({
        "device_id": device_id,
        "device_secret": old_secret,
    });

    let response = client()?
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![import_registration_text, activate_registration, rotate_device_credential, device_get, device_get_preview])
        .run(tauri::generate_context!())
        .expect("error while running INFO Desktop");
}
