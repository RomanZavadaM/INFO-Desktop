# INFO Desktop — technology choice

## Current decision
Primary implementation: **Tauri 2 + TypeScript + React**.

This decision was made after an explicit owner discussion on 2026-10-20.

## Supported product line
Primary targets:
- Windows 10/11 x64;
- macOS Apple Silicon / arm64;
- macOS Intel / x86_64;
- Linux x64.

Windows 7 is **not a required target**. It may be reconsidered later as a separate legacy client only if there is a real operational need.

## Why
INFO Desktop is intended as a long-lived high-volume newsroom workstation: large queues, filters/sorting, multi-pane work, media, later bulk actions, keyboard-first work and future updater support.

## Read-only network boundary
The first native layer exposes one narrow command: `readonly_get`.
It accepts only http/https, always performs GET, may attach a bearer token, and cannot publish or mutate server state.

## Significant-decision rule
Future choices that materially affect supported operating systems, product architecture, server protocol, storage, security or compatibility must be discussed with the owner before becoming project policy.
