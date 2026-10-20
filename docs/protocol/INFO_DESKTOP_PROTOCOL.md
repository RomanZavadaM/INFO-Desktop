# INFO Desktop Protocol

**Identifier:** `info-desktop-protocol/1`  
**Status:** draft / read-only subset

The protocol is semantic: a compatible server may use SQLite, PostgreSQL or another backend. Desktop must not depend on database/table names.

## Base path

`/api/info-desktop/v1`

## Read-only endpoints

- `GET /handshake`
- `GET /status`
- `GET /channels`
- `GET /queue?limit=<n>&cursor=<cursor>&channel=<code>&bucket=<bucket>`
- `GET /queue/{id}`
- media descriptors returned by item detail; the exact media fetch form is negotiated by capability/descriptor

No mutation endpoint is part of INFO Desktop 0.1.

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

Desktop renders only capabilities declared by the server.

## Channels

Each enabled editorial/publication channel may expose:

```json
{
  "code": "world",
  "name": "World",
  "language": "uk",
  "auto_publish": true,
  "pending": 26
}
```

A compatible server may omit optional fields.

## Queue page

The queue endpoint represents the **prepared editorial view**, not a live full-queue recalculation.

```json
{
  "items": [
    {
      "id": 3054,
      "event_id": 8123,
      "channel": "world",
      "channel_name": "World",
      "title": "…",
      "created_at": "…",
      "first_published_at": "…",
      "state": "pending",
      "bucket": "review",
      "quality_score": 4.8,
      "auto_ok": false,
      "issues": [],
      "language": "uk",
      "media_count": 1
    }
  ],
  "next_cursor": null,
  "counts": {
    "all": 98,
    "review": 42,
    "quick": 20,
    "weak": 26,
    "auto": 10
  }
}
```

### Editorial buckets

Protocol v1 recognizes the current general meanings:
- `review` — needs editorial decision;
- `quick` — can be reviewed quickly;
- `weak` — weak/doubtful;
- `auto` — prepared for automatic handling;
- `all` — filter value, not necessarily stored as an item bucket.

Servers may localize display labels; the protocol uses stable codes.

## Item detail

The item endpoint may expose:

```json
{
  "id": 3054,
  "event_id": 8123,
  "channel": "world",
  "channel_name": "World",
  "title": "Event title",
  "draft_text": "Prepared publication text",
  "state": "pending",
  "bucket": "review",
  "quality_score": 4.8,
  "auto_ok": false,
  "issues": [],
  "winner_channel": "world",
  "eligible_channels": ["world"],
  "event": {
    "url": "https://example.org/original",
    "first_source": "source-key",
    "first_published_at": "…",
    "topics": ["world"],
    "sources": ["source-key"]
  },
  "materials": [
    {
      "id": 9911,
      "source": "source-key",
      "source_name": "Example source",
      "url": "https://example.org/item",
      "title": "Original title",
      "summary": "Original summary",
      "published_at": "…",
      "language": "en",
      "translation_state": "ready",
      "translated_title": "…",
      "translated_summary": "…",
      "media": [
        {
          "kind": "photo",
          "mime_type": "image/jpeg",
          "url": "/api/info-desktop/v1/media/9911"
        }
      ]
    }
  ]
}
```

The server may add fields. Clients must ignore unknown fields.

## Current INFO mapping principle

The current INFO server already maintains a prepared editorial index and prepared queue draft. A protocol adapter should reuse these read-only prepared values rather than rerun routing, quality scoring or queue classification during a Desktop request.

For current INFO this means:
- queue ordering/filtering comes from the prepared editorial view;
- quality bucket/score/issues/auto state come from prepared editorial metadata;
- publication text comes from the already prepared queue draft;
- event provenance comes from event/source data;
- source text, translation state and media come from the event's underlying source materials.

This is a behavior contract, not a requirement to use the current INFO database schema.

## Media

The protocol must preserve real media dimensions/types/sizes where available.

A server must not expose its local filesystem paths to Desktop. Local cached media is served through a controlled read-only media route or another authenticated descriptor. Remote source media may be represented by a safe URL if the server policy allows it.

The exact authenticated media-fetch mechanism remains to be finalized before live acceptance.

## Pagination and scale

The queue endpoint must be bounded. A client may request a page/window and filters, but should not force the server to materialize or recalculate the whole queue on every click.

Protocol v1 permits cursor or offset implementation internally; clients should prefer the server-provided `next_cursor` when present.

## Authentication

Credentials are device/user secrets and must not be committed to Git or built into releases.

Bearer authentication is permitted for the first protocol version. Production transport requirements (for example HTTPS) are server policy and will be fixed before stable use.

## Compatibility

A client rejects an incompatible protocol identifier instead of guessing server semantics.

## Future write capabilities

Reserved for later discussion:
- `item.publish`
- `item.skip`
- `item.postpone`
- `item.move`
- `item.edit`
- `bulk.*`
- `admin.*`

They are **not** part of 0.1 read-only acceptance.
