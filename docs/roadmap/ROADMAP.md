# INFO Desktop — implementation roadmap

## A. Desktop foundation — DOING

Goal: cross-platform application skeleton that is independently buildable.

Acceptance:
- Windows 10/11 x64 build;
- macOS arm64 build;
- macOS x86_64 build;
- Linux x64 build;
- seven UI languages;
- no production secrets;
- native network surface remains GET-only.

## B. Real read-only protocol — DOING

Goal: show real INFO editorial state without slowing Telegram.

Acceptance:
- prepared editorial queue;
- channel counts;
- prepared draft;
- editorial score/bucket/issues;
- source materials/translations;
- media metadata/private cached-photo preview;
- bounded pagination;
- no synchronous quality/routing recalculation on clicks.

Server candidate: private INFO draft PR #18.

## C. Registration v1 — NEXT

Goal: one-file activation of a new Desktop device.

Work:
- canonical registration file parser;
- file validation/version/expiry;
- device key generation;
- one-time registration token;
- device registry model;
- enrollment request/response contract;
- revoke semantics;
- safe local secret storage;
- audit fields.

No production port/firewall changes in this stage.

## D. Protected transport — PLANNED

Leading candidate: WireGuard-style per-device public-key transport.

Work:
- choose final transport;
- choose high UDP port;
- server listener/peer registration;
- device internal address allocation;
- API binds only to private transport/loopback;
- device revoke cuts transport access;
- recovery/rotation documentation.

This stage requires explicit owner approval immediately before production network changes.

## E. Real LIVE acceptance — PLANNED

- create one registration grant for Roman-PC;
- import it into Desktop;
- establish protected transport;
- read LIVE queue;
- inspect real text/media/translation edge cases;
- verify Telegram remains unaffected;
- measure queue/card/media latency;
- keep all server actions read-only.

## F. Editorial write model — NOT STARTED

Only after read-only acceptance:
- roles;
- item.edit;
- item.publish;
- item.skip;
- postpone/move;
- bulk operations;
- action audit.

Each write operation must be a server business operation, never direct DB mutation from Desktop.

## G. Product release discipline — PLANNED

Follow the Taxo-style product process:
- synchronized version;
- CI;
- immutable release;
- Windows/macOS/Linux assets;
- checksums;
- user guide;
- project state/worklog;
- Google Drive release copy where required.
