use keyring::Entry;
use serde::Serialize;
use std::collections::HashMap;

pub const REGISTRATION_ID: &str = "INFO-DESKTOP-REGISTRATION/1";
const REGISTRATION_SECRET_SERVICE: &str = "INFO Desktop registration grant";
const DEVICE_SECRET_SERVICE: &str = "INFO Desktop device credential";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistrationFile {
    pub server: String,
    pub port: u16,
    pub registration_id: String,
    pub registration_token: String,
    pub expires_at: String,
    pub instance: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RegistrationSummary {
    pub server: String,
    pub port: u16,
    pub registration_id: String,
    pub expires_at: String,
    pub instance: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct StoredDeviceCredential {
    pub device_id: String,
    pub storage: String,
}

fn nonempty(map: &HashMap<String, String>, key: &str) -> Result<String, String> {
    map.get(key)
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .ok_or_else(|| format!("Missing registration field: {key}"))
}

fn valid_id(value: &str) -> bool {
    (4..=128).contains(&value.len())
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b':'))
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
    let port = nonempty(&fields, "port")?
        .parse::<u16>()
        .map_err(|_| "Registration port must be 1..65535".to_string())?;
    if port == 0 {
        return Err("Registration port must be 1..65535".to_string());
    }

    let registration_id = nonempty(&fields, "registration_id")?;
    if !valid_id(&registration_id) {
        return Err("Invalid registration_id".to_string());
    }

    let expires_at = nonempty(&fields, "expires_at")?;
    // Server is authoritative for expiry. Client keeps the timestamp for UI
    // and avoids inventing timezone/date semantics beyond requiring a value.

    Ok(RegistrationFile {
        server,
        port,
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

pub fn import_registration(text: &str) -> Result<RegistrationSummary, String> {
    let parsed = parse_registration(text)?;
    let entry = Entry::new(REGISTRATION_SECRET_SERVICE, &parsed.registration_id)
        .map_err(|e| format!("Secure credential store unavailable: {e}"))?;
    entry
        .set_secret(parsed.registration_token.as_bytes())
        .map_err(|e| format!("Cannot store registration grant securely: {e}"))?;

    Ok(RegistrationSummary {
        server: parsed.server,
        port: parsed.port,
        registration_id: parsed.registration_id,
        expires_at: parsed.expires_at,
        instance: parsed.instance,
    })
}

pub fn registration_token(registration_id: &str) -> Result<String, String> {
    if !valid_id(registration_id) {
        return Err("Invalid registration_id".to_string());
    }
    let entry = Entry::new(REGISTRATION_SECRET_SERVICE, registration_id)
        .map_err(|e| format!("Secure credential store unavailable: {e}"))?;
    let raw = entry
        .get_secret()
        .map_err(|e| format!("Registration grant is unavailable: {e}"))?;
    String::from_utf8(raw).map_err(|_| "Stored registration grant is invalid".to_string())
}

pub fn store_device_credential(
    device_id: &str,
    device_secret: &str,
) -> Result<StoredDeviceCredential, String> {
    if !valid_id(device_id) || device_secret.trim().len() < 24 {
        return Err("Invalid device credential".to_string());
    }
    let entry = Entry::new(DEVICE_SECRET_SERVICE, device_id)
        .map_err(|e| format!("Secure credential store unavailable: {e}"))?;
    entry
        .set_secret(device_secret.trim().as_bytes())
        .map_err(|e| format!("Cannot store device credential securely: {e}"))?;
    Ok(StoredDeviceCredential {
        device_id: device_id.to_string(),
        storage: "os-secure-store".to_string(),
    })
}

pub fn device_secret(device_id: &str) -> Result<String, String> {
    if !valid_id(device_id) {
        return Err("Invalid device_id".to_string());
    }
    let entry = Entry::new(DEVICE_SECRET_SERVICE, device_id)
        .map_err(|e| format!("Secure credential store unavailable: {e}"))?;
    let raw = entry
        .get_secret()
        .map_err(|e| format!("Device credential is unavailable: {e}"))?;
    String::from_utf8(raw).map_err(|_| "Stored device credential is invalid".to_string())
}

pub fn forget_registration_token(registration_id: &str) -> Result<(), String> {
    let entry = Entry::new(REGISTRATION_SECRET_SERVICE, registration_id)
        .map_err(|e| format!("Secure credential store unavailable: {e}"))?;
    match entry.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("Cannot remove registration grant: {e}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid() -> String {
        format!(
            "{REGISTRATION_ID}\n\nserver = info.example.test\nport = 47443\nregistration_id = reg-123\nregistration_token = secret-456-secret-789\nexpires_at = 2099-10-21T12:00:00+03:00\ninstance = primary\n"
        )
    }

    #[test]
    fn parses_simple_registration() {
        let parsed = parse_registration(&valid()).unwrap();
        assert_eq!(parsed.server, "info.example.test");
        assert_eq!(parsed.port, 47443);
        assert_eq!(parsed.instance, "primary");
    }

    #[test]
    fn rejects_url_in_server_field() {
        let text = valid().replace("info.example.test", "https://info.example.test");
        assert!(parse_registration(&text).is_err());
    }

    #[test]
    fn rejects_duplicate_secret_field() {
        let text = format!("{}registration_token = second\n", valid());
        assert!(parse_registration(&text).is_err());
    }

    #[test]
    fn rejects_wrong_header() {
        assert!(parse_registration("INFO-DESKTOP-REGISTRATION/2\n").is_err());
    }
}

pub fn take_registration_token(registration_id: &str) -> Result<String, String> {
    let token = registration_token(registration_id)?;
    forget_registration_token(registration_id)?;
    Ok(token)
}
