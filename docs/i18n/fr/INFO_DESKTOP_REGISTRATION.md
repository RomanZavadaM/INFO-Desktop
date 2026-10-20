# Enregistrement du périphérique

**Identifier:** `info-desktop-registration/1`

User flow: install Desktop → import registration file → activate device → work.

The registration file carries server/port, one-time registration id/token, expiry and the public INFO Desktop CA. After enrollment the server returns `device_id` and a high-entropy `device_secret`.

The reusable secret is stored only in the OS secure credential store. The server stores only its hash. Credentials can be rotated and a single device can be revoked independently.

Current scopes include read scopes plus `item.write`, `item.create` and `publish.write` where authorized.

The public CA in the registration file is not a secret and is used only by INFO Desktop transport; it is not installed globally into the operating system.
