mod device_keys;
mod registration;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use serde::Serialize;
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
    if parsed.scheme() != "https" && parsed.scheme() != "http" {
        return Err("Only http:// and https:// INFO servers are allowed".to_string());
    }
    Ok(parsed)
}

fn client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .user_agent("INFO-Desktop/0.1.0-dev")
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()
        .map_err(|error| error.to_string())
}

#[command]
async fn readonly_get(url: String, token: String) -> Result<ReadonlyResponse, String> {
    let parsed = checked_url(&url)?;
    let mut request = client()?
        .get(parsed)
        .header(reqwest::header::ACCEPT, "application/json");

    if !token.trim().is_empty() {
        request = request.bearer_auth(token.trim());
    }

    let response = request.send().await.map_err(|error| error.to_string())?;
    let status = response.status().as_u16();
    let body = response.text().await.map_err(|error| error.to_string())?;

    Ok(ReadonlyResponse { status, body })
}

#[command]
async fn readonly_get_preview(
    url: String,
    token: String,
) -> Result<ReadonlyBinaryResponse, String> {
    let parsed = checked_url(&url)?;
    let mut request = client()?
        .get(parsed)
        .header(reqwest::header::ACCEPT, "image/*");

    if !token.trim().is_empty() {
        request = request.bearer_auth(token.trim());
    }

    let response = request.send().await.map_err(|error| error.to_string())?;
    let status = response.status().as_u16();
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();

    if status < 200 || status >= 300 {
        return Ok(ReadonlyBinaryResponse {
            status,
            content_type,
            body_base64: String::new(),
        });
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

#[command]
fn ensure_device_key(registration_id: String) -> Result<device_keys::DevicePublicIdentity, String> {
    device_keys::ensure_device_key(&registration_id)
}

#[command]
fn forget_device_key(registration_id: String) -> Result<(), String> {
    device_keys::forget_device_key(&registration_id)
}

#[command]
fn import_registration_text(text: String) -> Result<registration::RegistrationSummary, String> {
    registration::import_registration(&text)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![readonly_get, readonly_get_preview, import_registration_text, ensure_device_key, forget_device_key])
        .run(tauri::generate_context!())
        .expect("error while running INFO Desktop");
}
