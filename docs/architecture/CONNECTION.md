# INFO Desktop — connection architecture

## Principle

Security complexity belongs primarily on the server. The editor/admin client
must remain a normal self-contained desktop application.

The user does not install:
- WireGuard/VPN;
- helper services;
- separate certificates;
- drivers;
- command-line networking tools.

## User path

```text
install INFO Desktop
→ import registration file once
→ activation
→ work
```

After activation, INFO Desktop remembers only:
- server HTTPS endpoint;
- non-secret device ID.

The reusable device secret is kept only in the operating-system credential store.

## Server path

```text
INFO Desktop
  │ HTTPS, high server-selected TCP port
  │ X-INFO-Device + bearer device secret
  ▼
server TLS/security gateway
  ▼
loopback INFO Desktop API
  ▼
prepared editorial state / SQLite readonly
```

The gateway may provide:
- TLS termination;
- rate limiting;
- connection logging;
- filtering;
- secret rotation/revoke integration;
- hiding internal INFO processes/ports.

Exact production gateway technology is deliberately a server deployment choice.

## Registration

The one-time registration endpoint is the only client operation that writes
server-side state. It creates a Desktop device record only.

All newsroom/editorial operations in 0.1-r1 remain GET-only.

## Rotation

The server is authoritative for credential lifetime.

Normal rotation should be invisible:
1. current credential authenticates;
2. server issues replacement;
3. Desktop stores replacement in OS secure storage;
4. old credential remains valid briefly;
5. server expires old credential.

## Read-only invariant

Ordinary GET requests:
- authenticate against device credential hashes;
- do not mutate editorial data;
- use SQLite read-only connections;
- never directly publish/edit/skip/delete.

Request/access audit belongs at the server gateway/application logging layer,
not as a DB write on every newsroom click.
