# INFO Desktop — connection architecture

## Principle

Security complexity belongs primarily on the server. The editor/admin client
must remain a normal self-contained desktop application.

The user does not install helper services, separate certificates, drivers or command-line networking tools.
The current production INFO gateway may remain reachable only through the owner-controlled VPN/network policy.
INFO Desktop must not change VPN, firewall or SSH configuration.

## User path

```text
install INFO Desktop
→ import registration file once
→ activation
→ work
```

After activation, INFO Desktop remembers the server HTTPS endpoint and non-secret device ID.
The reusable device secret is kept only in the operating-system credential store.

## Server path

```text
INFO Desktop
  │ HTTPS
  │ X-INFO-Device + bearer device secret
  ▼
server TLS/security gateway
  ▼
loopback INFO Desktop API
  ▼
prepared editorial state / controlled write services
```

The gateway may provide TLS termination, rate limiting, connection logging, filtering, secret rotation/revoke integration and hiding of internal INFO processes/ports.
Exact production gateway technology is deliberately a server deployment choice.

## Registration and scoped actions

The one-time registration endpoint creates a Desktop device record and exchanges the registration token for a reusable device credential.
Normal newsroom reads use authenticated GET requests. Mutating editorial operations use authenticated POST requests and are accepted only when the server grants the required scope.
Current Desktop capabilities include scoped save / skip / defer / manual create, plus a separately scoped publish action.
Public publishing is never assumed by the client. Production publish acceptance is performed only with explicit owner authorization.

## Rotation

The server is authoritative for credential lifetime.

Normal rotation should be invisible:
1. current credential authenticates;
2. server issues replacement;
3. Desktop stores replacement in OS secure storage;
4. old credential remains valid briefly;
5. server expires old credential.

## Read invariant

Ordinary GET requests authenticate against device credentials, do not mutate editorial data and use server-controlled read paths.
All state-changing requests go through explicit API endpoints and server-side permission checks.
Request/access audit belongs at the server gateway/application logging layer, not as an uncontrolled direct DB write from the Desktop client.
