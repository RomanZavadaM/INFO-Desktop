# INFO Desktop — local credentials

INFO Desktop does not require a separate VPN client or user-managed key pair.

The client may hold only:
- one-time registration token until activation;
- server-issued device secret after activation.

Both use the operating-system credential store through the Rust keyring layer.

Target stores:
- Windows: Windows credential facilities;
- macOS: Keychain;
- Linux desktop: Secret Service/keyring where available.

There is no plaintext fallback. If secure storage is unavailable, activation
must fail with a clear message rather than writing a secret into JSON/INI files.

The server controls:
- credential creation;
- expiry;
- revoke;
- rotation;
- scopes;
- audit.
