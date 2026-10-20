use chrono::{DateTime, FixedOffset, Utc};
use keyring::Entry;
use serde::Serialize;
use std::collections::HashMap;

pub const REGISTRATION_ID: &str = "INFO-DESKTOP-REGISTRATION/1";
const REGISTRATION_SECRET_SERVICE: &str = "INFO Desktop registration grant";
const BOOTSTRAP_KEY_SERVICE: &str = "INFO Desktop bootstrap transport key";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistrationFile {
    pub server: String,
    pub transport: String,
    pub port: u16,
    pub server_public_key: String,
    pub bootstrap_private_key: String,
    pub bootstrap_address: String,
    pub private_server: String,
    pub registration_id: String,
    pub registration_token: String,
    pub expires_at: String,
    pub instance: String,
}

fn nonempty(map: &HashMap<String, String>, key: &str) -> Result<String, String> {
    map.get(key)
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .ok_or_else(|| format!("Missing registration field: {key}"))
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RegistrationSummary {
    pub server: String,
    pub transport: String,
    pub port: u16,
    pub server_public_key: String,
    pub bootstrap_address: String,
    pub private_server: String,
    pub registration_id: String,
    pub expires_at: String,
    pub instance: String,
}

pub fn import_registration(text: &str) -> Result<RegistrationSummary, String> {
    let parsed = parse_registration(text)?;
    let entry = Entry::new(REGISTRATION_SECRET_SERVICE, &parsed.registration_id)
        .map_err(|e| format!("Secure credential store unavailable: {e}"))?;
    entry
        .set_secret(parsed.registration_token.as_bytes())
        .map_err(|e| format!("Cannot store registration grant securely: {e}"))?;

    let bootstrap_raw = base64::engine::general_purpose::STANDARD
        .decode(parsed.bootstrap_private_key.as_bytes())
        .map_err(|_| "Invalid bootstrap private key".to_string())?;
    let bootstrap_entry = Entry::new(BOOTSTRAP_KEY_SERVICE, &parsed.registration_id)
        .map_err(|e| format!("Secure credential store unavailable: {e}"))?;
    bootstrap_entry
        .set_secret(&bootstrap_raw)
        .map_err(|e| format!("Cannot store bootstrap key securely: {e}"))?;

    Ok(RegistrationSummary {
        server: parsed.server,
        transport: parsed.transport,
        port: parsed.port,
        server_public_key: parsed.server_public_key,
        bootstrap_address: parsed.bootstrap_address,
        private_server: parsed.private_server,
        registration_id: parsed.registration_id,
        expires_at: parsed.expires_at,
        instance: parsed.instance,
    })
}

pub fn registration_token(registration_id: &str) -> Result<String, String> {
    let entry = Entry::new(REGISTRATION_SECRET_SERVICE, registration_id)
        .map_err(|e| format!("Secure credential store unavailable: {e}"))?;
    let raw = entry
        .get_secret()
        .map_err(|e| format!("Registration grant is unavailable: {e}"))?;
    String::from_utf8(raw).map_err(|_| "Stored registration grant is invalid".to_string())
}

pub fn forget_registration_token(registration_id: &str) -> Result<(), String> {
    let entry = Entry::new(REGISTRATION_SECRET_SERVICE, registration_id)
        .map_err(|e| format!("Secure credential store unavailable: {e}"))?;
    match entry.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("Cannot remove registration grant: {e}")),
    }
}

fn valid_wg_key(value: &str) -> bool {
    use base64::Engine;
    match base64::engine::general_purpose::STANDARD.decode(value.as_bytes()) {
        Ok(raw) => raw.len() == 32,
        Err(_) => false,
    }
}

pub fn parse_registration(text: &str) -> Result<RegistrationFile, String> {
    let mut lines = text.lines();
    let header = lines
        .find(|line| !line.trim().is_empty())
        .map(str::trim)
        .unwrap_or("");

    if header != REGISTRATION_ID {
        return Err("Unsupported registration file version".to_string());
    }

    let mut fields = HashMap::new();
    for raw in lines {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("Invalid registration line: {line}"))?;
        let key = key.trim().to_ascii_lowercase();
        let value = value.trim().to_string();
        if key.is_empty() || value.is_empty() {
            return Err(format!("Invalid registration line: {line}"));
        }
        if fields.insert(key.clone(), value).is_some() {
            return Err(format!("Duplicate registration field: {key}"));
        }
    }

    let server = nonempty(&fields, "server")?;
    if server.contains('/') || server.contains(' ') {
        return Err("Registration server must be a host name or IP, not a URL".to_string());
    }

    let transport = nonempty(&fields, "transport")?;
    if transport != "wireguard-bootstrap-v1" {
        return Err("Unsupported registration transport".to_string());
    }
    let port_raw = nonempty(&fields, "port")?;
    let port = port_raw
        .parse::<u16>()
        .map_err(|_| "Registration port must be 1..65535".to_string())?;
    if port == 0 {
        return Err("Registration port must be 1..65535".to_string());
    }

    let expires_at = nonempty(&fields, "expires_at")?;
    let expiry: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(&expires_at)
        .map_err(|_| "expires_at must be RFC3339 with timezone".to_string())?;
    if expiry.with_timezone(&Utc) <= Utc::now() {
        return Err("Registration file has expired".to_string());
    }

    let server_public_key = nonempty(&fields, "server_public_key")?;
    if !valid_wg_key(&server_public_key) {
        return Err("Invalid server_public_key".to_string());
    }
    let bootstrap_private_key = nonempty(&fields, "bootstrap_private_key")?;
    if !valid_wg_key(&bootstrap_private_key) {
        return Err("Invalid bootstrap_private_key".to_string());
    }
    let bootstrap_address = nonempty(&fields, "bootstrap_address")?;
    if !bootstrap_address.contains('/') || bootstrap_address.contains(' ') {
        return Err("Invalid bootstrap_address".to_string());
    }
    let private_server = nonempty(&fields, "private_server")?;
    if private_server.contains('/') || private_server.contains(' ') {
        return Err("Invalid private_server".to_string());
    }

    let registration_id = nonempty(&fields, "registration_id")?;
    if registration_id.len() < 4
        || registration_id.len() > 128
        || !registration_id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b':'))
    {
        return Err("Invalid registration_id".to_string());
    }

    Ok(RegistrationFile {
        server,
        transport,
        port,
        server_public_key,
        bootstrap_private_key,
        bootstrap_address,
        private_server,
        registration_id,
        registration_token: nonempty(&fields, "registration_token")?,
        expires_at,
        instance: fields
            .get("instance")
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| "primary".to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid(expiry: &str) -> String {
        format!(
            "{REGISTRATION_ID}\n\nserver = info.example.test\ntransport = wireguard-bootstrap-v1\nport = 47000\nserver_public_key = AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8=\nbootstrap_private_key = HyAeHRwbGhkYFxYVFBMSERAPDg0MCwoJCAcGBQQDAgE=\nbootstrap_address = 10.77.0.240/32\nprivate_server = 10.77.0.1\nregistration_id = reg-123\nregistration_token = secret-456\nexpires_at = {expiry}\ninstance = primary\n"
        )
    }

    #[test]
    fn parses_valid_registration() {
        let parsed = parse_registration(&valid("2099-10-21T12:00:00+03:00")).unwrap();
        assert_eq!(parsed.server, "info.example.test");
        assert_eq!(parsed.port, 47000);
        assert_eq!(parsed.instance, "primary");
        assert_eq!(parsed.transport, "wireguard-bootstrap-v1");
        assert_eq!(parsed.bootstrap_address, "10.77.0.240/32");
        assert_eq!(parsed.private_server, "10.77.0.1");
    }

    #[test]
    fn rejects_expired_registration() {
        let error = parse_registration(&valid("2020-01-01T00:00:00+00:00")).unwrap_err();
        assert!(error.contains("expired"));
    }

    #[test]
    fn rejects_url_in_server_field() {
        let text = valid("2099-10-21T12:00:00+03:00")
            .replace("info.example.test", "https://info.example.test");
        assert!(parse_registration(&text).is_err());
    }

    #[test]
    fn rejects_duplicate_secret_field() {
        let text = format!(
            "{}registration_token = second\n",
            valid("2099-10-21T12:00:00+03:00")
        );
        assert!(parse_registration(&text).is_err());
    }

    #[test]
    fn rejects_wrong_header() {
        assert!(parse_registration("INFO-DESKTOP-REGISTRATION/2\n").is_err());
    }
}
