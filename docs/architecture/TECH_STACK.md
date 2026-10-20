# INFO Desktop — technology choice

## Current decision
Primary implementation: **Tauri 2 + TypeScript + React**.

## Supported product line
- Windows 10/11 x64;
- macOS Apple Silicon / arm64;
- macOS Intel / x86_64;
- Linux x64.

Windows 7 is not required. A separate legacy client may be reconsidered only for a real need.

## Read-only network boundary

The first native layer exposes only two operations and both are HTTP GET:
- JSON GET for protocol data;
- authenticated image-preview GET with an 8 MiB in-memory safety limit.

There is no POST/PUT/PATCH/DELETE transport in 0.1.

Private media previews are fetched through the native layer so the bearer token is not embedded in a public image URL. The returned preview is shown from memory as a data URL; INFO Desktop does not automatically save it to disk.

Video is not automatically transferred in the 0.1 preview path.

## Third-party media policy

Desktop treats media as attributed source material, not as product-owned media:
- provenance/source URL remains visible;
- server policy controls whether preview is available;
- `temporary_cache` means an authenticated editorial preview may be read from an existing technical cache;
- `external_only` means Desktop shows metadata/source only;
- original downloads are not part of 0.1.

## Significant-decision rule
Future choices that materially affect supported OSes, product architecture, server protocol, storage, media rights handling, security or compatibility must be discussed with the owner first.
