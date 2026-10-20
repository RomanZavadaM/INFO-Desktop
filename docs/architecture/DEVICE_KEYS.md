# INFO Desktop — device key storage

## Accepted behavior

A registration file authorizes enrollment but does not contain the permanent device private key.

When Desktop prepares a device identity:
1. Rust/Tauri validates the registration ID.
2. If this installation already has a key for that registration ID, it reuses it.
3. Otherwise it generates a fresh X25519 static key locally.
4. The 32-byte private key is written to the operating-system credential store.
5. Only the Base64 public key is returned to the UI/enrollment layer.

## Secure stores

INFO Desktop uses the platform credential abstraction provided by the Rust keyring ecosystem:
- Windows → Windows Credential Manager;
- macOS → Keychain;
- Linux/*nix → Secret Service where available.

The application does not write the device private key into:
- registration files;
- Git;
- Google Drive;
- ordinary JSON/INI settings;
- UI state.

## WireGuard relationship

X25519 was selected because the protected-transport direction is WireGuard-compatible. This does not by itself deploy or configure WireGuard.

## Local removal

A local forget operation removes only the local credential. Server-side revoke is a separate operation and remains the authoritative way to disable a registered device.

## Failure policy

If the OS secure credential store cannot be initialized or cannot save the key, enrollment fails. INFO Desktop must not silently fall back to a plaintext private-key file.
