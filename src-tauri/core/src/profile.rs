//! Профіль з'єднання із сервером INFO: адреса, порт, номер пристрою, довірений сертифікат сервера. Секретів не містить:
//! ключ пристрою лежить лише у сховищі ОС. Профіль пише вікно застосунку після підключення; його читає консоль.

use serde::{Deserialize, Serialize};
use std::path::Path;

pub const API: &str = "/api/info-desktop/v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    pub server: String,
    pub port: u16,
    pub device_id: String,
    pub server_ca_der: String,
}

impl Profile {
    pub fn url(&self, path: &str) -> String {
        format!("https://{}:{}{}{}", self.server.trim(), self.port, API, path)
    }

    /// Із адреси сервера у вигляді `https://адреса:порт` (так її тримає вікно застосунку).
    pub fn from_server_url(server_url: &str, device_id: &str, server_ca_der: &str) -> Result<Profile, String> {
        let parsed = url::Url::parse(server_url.trim().trim_end_matches('/')).map_err(|_| "Invalid server URL".to_string())?;
        if parsed.scheme() != "https" {
            return Err("INFO Desktop requires https:// server connections".to_string());
        }
        let host = parsed.host_str().ok_or_else(|| "Server URL has no host".to_string())?.to_string();
        let port = parsed.port_or_known_default().ok_or_else(|| "Server URL has no port".to_string())?;
        Ok(Profile {
            server: host,
            port,
            device_id: device_id.trim().to_string(),
            server_ca_der: server_ca_der.trim().to_string(),
        })
    }
}

pub fn load_profile(path: &Path) -> Result<Profile, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|_| format!("Немає профілю з'єднання: {} (підключіться у вікні застосунку або profile set)", path.display()))?;
    serde_json::from_str(&text).map_err(|_| "Профіль з'єднання пошкоджений".to_string())
}

pub fn save_profile(path: &Path, profile: &Profile) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string_pretty(profile).map_err(|e| e.to_string())?;
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, text).map_err(|e| e.to_string())?;
    std::fs::rename(&tmp, path).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_profile_from_server_url() {
        let p = Profile::from_server_url(" https://10.8.0.1:47443/ ", " dev-1 ", " QUFB ").unwrap();
        assert_eq!(p, Profile { server: "10.8.0.1".into(), port: 47443, device_id: "dev-1".into(), server_ca_der: "QUFB".into() });
        assert_eq!(Profile::from_server_url("https://info.example.net", "d", "c").unwrap().port, 443);
        assert!(Profile::from_server_url("http://10.8.0.1:47443", "d", "c").is_err());
        assert!(Profile::from_server_url("not a url", "d", "c").is_err());
    }

    #[test]
    fn profile_url_uses_https_api_base() {
        let p = Profile { server: " 10.8.0.1 ".into(), port: 47443, device_id: "d".into(), server_ca_der: "x".into() };
        assert_eq!(p.url("/status"), "https://10.8.0.1:47443/api/info-desktop/v1/status");
    }

    #[test]
    fn profile_round_trips_without_secrets() {
        let dir = std::env::temp_dir().join(format!("info-desktop-core-profile-{}", std::process::id()));
        let file = dir.join("connection.json");
        let p = Profile { server: "10.8.0.1".into(), port: 47443, device_id: "dev-1".into(), server_ca_der: "QUFB".into() };
        save_profile(&file, &p).unwrap();
        assert_eq!(load_profile(&file).unwrap(), p);
        assert!(!std::fs::read_to_string(&file).unwrap().contains("secret"));
        std::fs::remove_dir_all(&dir).ok();
        assert!(load_profile(&file).is_err());
    }
}
