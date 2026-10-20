# INFO Desktop Registration Protocol v1

**Identifier:** `info-desktop-registration/1`  
**Status:** accepted design / not deployed

## User-facing rule

The editor installs only INFO Desktop.

No separate VPN, WireGuard, driver, certificate manager or helper application is
required from the user.

User flow:

```text
install INFO Desktop
→ import registration file
→ activate device
→ work
```

## Connection model

```text
INFO Desktop
    │ TLS on server-selected high TCP port
    │ device_id + device_secret
    ▼
server security gateway
    ▼
INFO Desktop API
```

Server-side infrastructure may be as strict or complex as needed. That
complexity is deliberately hidden from the editor/admin client.

## Registration file

The file is delivered through a separate trusted path such as SSH/SCP, USB or
another owner-approved channel.

```text
INFO-DESKTOP-REGISTRATION/1

server = info.example.net
port = 47443
registration_id = reg-...
registration_token = <one-time secret>
expires_at = 2026-10-21T12:00:00+03:00
instance = primary
```

The registration token is only for first activation. It is not the permanent
Desktop credential.

## Activation

1. Desktop imports the file.
2. The one-time registration token is stored in the OS credential store.
3. Desktop contacts the registration endpoint over TLS.
4. Server validates and consumes the registration grant.
5. Server creates a device record and returns:
   - `device_id`;
   - high-entropy `device_secret`;
   - credential expiry/rotation metadata.
6. Desktop stores `device_secret` in the OS credential store.
7. One-time registration token is deleted locally.
8. Future API requests authenticate as that device.

The server stores only hashes of reusable device secrets.

## Rotation

Credential rotation is server-controlled and should normally be invisible to the
editor.

Planned flow:
- server announces rotation required / near expiry;
- authenticated Desktop asks for replacement credential;
- new credential is stored locally;
- old credential has a short grace period and is then invalidated.

Lost/stolen device:
- server revokes that single `device_id`;
- other Desktop installations continue working.

## Initial permissions

0.1-r1:
- `queue.read`
- `item.read`
- `media.preview`

There is no edit/publish/delete permission in the first product stage.

## Security proportion

INFO mostly handles public source material and editorial derivatives. The design
therefore prioritizes:
- preventing unauthorized control of INFO;
- avoiding generic Internet scanning/noise;
- independent device revoke;
- encrypted transport;
- simple editor UX.

It intentionally avoids client-side VPN complexity that does not materially
improve the first read-only use case.

## Production boundary

The exact public hostname, high TCP port, TLS termination, firewall/rate-limit
configuration and credential rotation period are server production decisions.
They are not hard-coded in the public Desktop repository.
