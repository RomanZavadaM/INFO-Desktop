# INFO Desktop Registration Protocol v1

**Identifier:** `info-desktop-registration/1`  
**Status:** design / not deployed

## Purpose

A user receives one registration file through a separate trusted delivery path (for example SSH/SCP, USB or another protected channel). Importing that file authorizes exactly one Desktop installation to enroll itself.

The registration file is **not** the permanent private key of the device.

## User experience

1. Open INFO Desktop.
2. Choose **Import registration**.
3. Select the registration file.
4. Desktop validates it.
5. Desktop generates its device key pair locally.
6. Desktop enrolls the public key with the server.
7. Desktop receives its protected-transport parameters and device identity.
8. Permanent private material is stored in the operating system secure credential facility.
9. The one-time registration grant becomes unusable.

Target user experience: **file → import → connected**.

## Registration file

Recommended human-readable extension: `.info-registration` or `.txt` during development.

Example semantic payload:

```text
INFO-DESKTOP-REGISTRATION/1

server = example.invalid
transport = protected-v1
port = 47000
server_public_key = <public key>
registration_id = <random id>
registration_token = <one-time secret>
expires_at = 2026-10-21T12:00:00+03:00
instance = primary
```

The exact serialized format may later become JSON or another signed container. The semantic fields are the contract.

## Rules

- registration token is random and single-use;
- registration expires;
- registration token is never reused as the permanent device credential;
- no device private key is shipped in the file;
- server private keys never appear in the file;
- the file may carry the server public key/fingerprint;
- Desktop generates the device private/public key pair locally;
- only the device public key is enrolled on the server;
- successful enrollment consumes the registration grant;
- failed/expired/revoked grants cannot be used.

## Device identity

After enrollment the server has a durable device record:

```text
device_id
display_name
device_public_key
transport_address
created_at
last_seen_at
status = active|revoked
scopes
```

Each installation has its own identity. A second PC must enroll separately.

## Revocation

Revoking a device invalidates only that device.

Examples:
- lost notebook → revoke that notebook;
- editor leaves → revoke that editor's devices;
- Roman-PC remains unaffected.

Revocation must not require changing Telegram tokens, SSH credentials or credentials of other Desktop installations.

## Transport model

The target model is a cryptographically authenticated private transport, with WireGuard as the current leading implementation candidate.

Conceptually:

```text
Desktop
  │ device cryptographic key
  │ high non-standard UDP port
  ▼
protected transport listener
  │
  ▼
private INFO network address
  │
  ▼
INFO Desktop API
```

The non-standard port reduces generic scan noise but is not treated as security. Security comes from cryptographic authentication.

The read-only HTTP API should not be directly exposed to the public Internet.

## Application authorization

Transport identity and application permission are separate layers.

Initial scopes:
- `queue.read`
- `item.read`
- `media.preview`

There is no publish/edit/delete scope in the 0.1 read-only product.

## Local secret storage

Permanent device private material must not be stored as plain project/config text.

Target stores:
- Windows: OS credential/key storage;
- macOS: Keychain;
- Linux: Secret Service/keyring or equivalent secure store.

Exact Tauri integration is a separate implementation slice.

## Media relationship

Registration/transport does not change the content policy.

For the initial product:
- cached source photos may be viewed for editorial work;
- their life cycle follows the editorial/cache policy, currently aligned with the approximately 48-hour material cycle;
- remote-only third-party originals are not automatically copied;
- video remains metadata-first;
- source/provenance remains visible.

## Security invariants

Even after device enrollment:
- API remains GET-only during 0.1;
- server opens SQLite read-only;
- no direct SQLite/SSH access from Desktop;
- media path is restricted to INFO media cache;
- local server filesystem paths are never returned;
- device/private keys and registration secrets are never committed to Git.

## Production boundary

This document does **not** authorize a production network change.

Before deployment, owner review is required for:
- exact protected-transport implementation;
- chosen public UDP port;
- firewall/network changes;
- service configuration;
- key provisioning/revocation operations;
- recovery procedure.
