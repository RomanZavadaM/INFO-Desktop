# INFO Desktop Protocol

**Identifier:** `info-desktop-protocol/1`  
**Status:** draft / read-only subset

## Base path

`/api/info-desktop/v1`

## Read-only endpoints

- `GET /handshake`
- `GET /status`
- `GET /channels`
- `GET /queue?limit=<n>&cursor=<cursor>`
- `GET /queue/{id}`

Media access is described by item/media descriptors returned by the server. Large binary content should not be embedded directly into queue JSON.

## Handshake

Minimum response:
```json
{
  "protocol": "info-desktop-protocol/1",
  "server_name": "INFO",
  "instance_name": "Example newsroom",
  "server_version": "20261019-2245",
  "timezone": "Europe/Kyiv",
  "capabilities": [
    "status.read",
    "channels.read",
    "queue.read",
    "item.read",
    "media.read"
  ],
  "languages": ["uk", "en"]
}
```

Desktop must render only capabilities actually declared by the server.

## Queue response

Minimum shape:
```json
{
  "items": [
    {
      "id": 123,
      "channel": "world",
      "channel_name": "Світ",
      "title": "…",
      "created_at": "…",
      "state": "pending",
      "language": "uk",
      "media_count": 1
    }
  ],
  "next_cursor": null
}
```

## Item detail

May include:
- original/prepared title and text;
- source/provenance;
- channel/routing;
- quality/editorial status;
- language and translation state;
- media descriptors;
- related/similar items;
- server-side warnings.

## Read-only guarantee

Protocol v1 subset used by INFO Desktop 0.1.x performs no mutation. Client code must not send POST/PUT/PATCH/DELETE during this phase.

## Authentication

Bearer token is allowed for the first protocol version, but credentials are installation/user secrets and must not be committed to Git or built into releases.

## Compatibility

A client must reject an incompatible protocol identifier instead of guessing server semantics.

## Future write capabilities

Reserved future capabilities include:
`item.publish`, `item.skip`, `item.postpone`, `item.move`, `item.edit`, `bulk.*`, `admin.*`.

Those are not part of the 0.1 read-only acceptance.
