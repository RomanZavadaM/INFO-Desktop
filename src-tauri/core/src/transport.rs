//! Спільний обмін із сервером INFO: лише https, довіра тільки до сертифіката сервера з реєстрації, ключ пристрою зі сховища ОС.
//! Цей модуль використовують і вікно застосунку, і консольний режим, щоб правила транспорту були в одному місці.

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use serde::Serialize;
use url::Url;

use crate::registration;

#[derive(Serialize)]
pub struct ReadonlyResponse {
    pub status: u16,
    pub body: String,
}

pub fn checked_url(url: &str) -> Result<Url, String> {
    let parsed = Url::parse(url).map_err(|_| "Invalid server URL".to_string())?;
    if parsed.scheme() != "https" {
        return Err("INFO Desktop requires https:// server connections".to_string());
    }
    Ok(parsed)
}

pub fn client_with_cert(server_ca_der: &str) -> Result<reqwest::Client, String> {
    let cert_bytes = BASE64
        .decode(server_ca_der.trim())
        .map_err(|_| "Invalid server CA certificate".to_string())?;
    let cert = reqwest::Certificate::from_der(&cert_bytes)
        .map_err(|_| "Invalid server CA certificate DER".to_string())?;
    reqwest::Client::builder()
        .user_agent("INFO-Desktop/0.1.2")
        .redirect(reqwest::redirect::Policy::none())
        .tls_built_in_root_certs(false)
        .add_root_certificate(cert)
        .build()
        .map_err(|error| error.to_string())
}

/// Читання з автентифікацією пристрою (ключ береться зі сховища ОС за номером пристрою).
pub async fn device_get(
    url: &str,
    device_id: &str,
    server_ca_der: &str,
) -> Result<ReadonlyResponse, String> {
    let parsed = checked_url(url)?;
    let secret = registration::device_secret(device_id)?;
    let response = client_with_cert(server_ca_der)?
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

/// Єдиний редакторський запис на сервер (усі зміни йдуть лише сюди; права перевіряє сервер).
pub async fn device_post(
    url: &str,
    device_id: &str,
    server_ca_der: &str,
    body: String,
) -> Result<ReadonlyResponse, String> {
    let parsed = checked_url(url)?;
    let secret = registration::device_secret(device_id)?;
    let response = client_with_cert(server_ca_der)?
        .post(parsed)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .header("X-INFO-Device", device_id)
        .bearer_auth(secret)
        .body(body)
        .send()
        .await
        .map_err(|error| error.to_string())?;
    let status = response.status().as_u16();
    let body = response.text().await.map_err(|error| error.to_string())?;
    Ok(ReadonlyResponse { status, body })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_only_https_urls() {
        assert!(checked_url("https://10.8.0.1:47443/api/info-desktop/v1/status").is_ok());
        assert!(checked_url("http://10.8.0.1:47443/x").is_err());
        assert!(checked_url("not a url").is_err());
    }

    #[test]
    fn rejects_broken_server_certificate() {
        assert!(client_with_cert("%%%not-base64%%%").is_err());
        assert!(client_with_cert("QUFBQQ==").is_err());
    }
}
