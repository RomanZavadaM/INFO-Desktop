# INFO Desktop Registration Protocol v1

**Identifier:** `info-desktop-registration/1`  
**Transport profile:** `wireguard-bootstrap-v1`  
**Status:** design / not deployed

## Core idea

The registration file is delivered through a separate trusted path such as SSH/SCP or USB.

It contains **a temporary bootstrap transport credential**, not the permanent Desktop device key.

This removes the bootstrap paradox: the server already knows the temporary peer's public key before the client ever connects.

## Flow

1. Server creates a short-lived bootstrap WireGuard key pair and one-time registration grant.
2. Server keeps the bootstrap **public** key and temporary allowed address.
3. Registration file receives the bootstrap **private** key, server public key, hidden/high UDP port, temporary address and one-time registration token.
4. The file is delivered separately (for example by SCP).
5. Desktop imports it and moves the bootstrap private key + registration token into the OS secure store.
6. Desktop independently generates its permanent device X25519 key pair locally.
7. Desktop uses the bootstrap key to establish the temporary protected tunnel.
8. Inside that tunnel it calls the one-purpose enrollment service and submits only its permanent public key plus the one-time token.
9. Server creates the permanent device record.
10. Bootstrap peer/grant is invalidated.
11. Desktop switches to its permanent key and deletes the bootstrap secret/token.
12. The source registration file is no longer needed.

## Registration file

Semantic example:

```text
INFO-DESKTOP-REGISTRATION/1

server = info.example.net
transport = wireguard-bootstrap-v1
port = 47xxx
server_public_key = <32-byte WireGuard public key, Base64>
bootstrap_private_key = <32-byte TEMPORARY private key, Base64>
bootstrap_address = 10.77.0.240/32
private_server = 10.77.0.1
registration_id = reg-...
registration_token = <one-time secret>
expires_at = 2026-10-21T12:00:00+03:00
instance = primary
```

## Secret distinction

**Temporary secret shipped in the file**
- bootstrap private key;
- one-time registration token;
- short lifetime;
- transported by an already trusted out-of-band path;
- destroyed after enrollment.

**Permanent device secret**
- generated locally by INFO Desktop;
- never shipped by server;
- never present in registration file;
- stored in OS secure credential storage.

## Why this model

The public Internet sees only the protected transport listener on a non-standard UDP port. The enrollment HTTP service itself does not need to be public.

Packets without a valid transport key are not an INFO API login attempt. The unusual port reduces scan noise, while cryptographic authentication supplies the actual security.

## Application scopes

Initial permanent device scopes:
- `queue.read`
- `item.read`
- `media.preview`

No editorial write scope exists in 0.1.

## Production boundary

This protocol design does not authorize:
- opening the UDP port;
- changing firewall;
- installing/configuring WireGuard on LIVE;
- creating system services;
- changing SSH/VPN.

Those are a separate owner-approved production step.
