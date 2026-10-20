use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use keyring::Entry;
use serde::Serialize;
use x25519_dalek::{PublicKey, StaticSecret};

const KEYRING_SERVICE: &str = "INFO Desktop device key";

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct DevicePublicIdentity {
    pub registration_id: String,
    pub public_key: String,
    pub created: bool,
    pub storage: String,
}

fn validate_registration_id(value: &str) -> Result<String, String> {
    let value = value.trim();
    if value.len() < 4 || value.len() > 128 {
        return Err("Invalid registration id length".to_string());
    }
    if !value
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b':'))
    {
        return Err("Invalid registration id characters".to_string());
    }
    Ok(value.to_string())
}

fn public_key(secret_bytes: [u8; 32]) -> String {
    let secret = StaticSecret::from(secret_bytes);
    let public = PublicKey::from(&secret);
    BASE64.encode(public.as_bytes())
}

pub fn ensure_device_key(registration_id: &str) -> Result<DevicePublicIdentity, String> {
    let registration_id = validate_registration_id(registration_id)?;
    let entry = Entry::new(KEYRING_SERVICE, &registration_id)
        .map_err(|e| format!("Secure credential store unavailable: {e}"))?;

    let (secret_bytes, created) = match entry.get_secret() {
        Ok(existing) => {
            let secret: [u8; 32] = existing
                .as_slice()
                .try_into()
                .map_err(|_| "Stored device key has invalid length".to_string())?;
            (secret, false)
        }
        Err(keyring::Error::NoEntry) => {
            let secret = StaticSecret::random();
            let bytes = secret.to_bytes();
            entry
                .set_secret(&bytes)
                .map_err(|e| format!("Cannot store device key securely: {e}"))?;
            (bytes, true)
        }
        Err(e) => {
            return Err(format!("Cannot read secure device key: {e}"));
        }
    };

    Ok(DevicePublicIdentity {
        registration_id,
        public_key: public_key(secret_bytes),
        created,
        storage: "os-secure-store".to_string(),
    })
}

pub fn forget_device_key(registration_id: &str) -> Result<(), String> {
    let registration_id = validate_registration_id(registration_id)?;
    let entry = Entry::new(KEYRING_SERVICE, &registration_id)
        .map_err(|e| format!("Secure credential store unavailable: {e}"))?;
    match entry.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("Cannot remove secure device key: {e}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unsafe_registration_ids() {
        assert!(validate_registration_id("../../secret").is_err());
        assert!(validate_registration_id("has space").is_err());
        assert!(validate_registration_id("").is_err());
    }

    #[test]
    fn derives_stable_32_byte_public_key() {
        let secret = [7u8; 32];
        let encoded = public_key(secret);
        let decoded = BASE64.decode(encoded).unwrap();
        assert_eq!(decoded.len(), 32);
        assert_eq!(encoded, public_key(secret));
    }
}
