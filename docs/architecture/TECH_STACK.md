# INFO Desktop — technology choice

## Decision

Primary implementation: **Python native desktop UI (Tk/ttk), one codebase**.

Rationale:
- preserves the Taxo-style platform matrix without requiring a browser engine;
- allows a separate Windows 7 compatibility build with Python 3.8.x;
- supports current Windows and macOS with a modern Python runtime;
- keeps the first client small and easy to package;
- protocol/network layer uses standard HTTP/JSON and is independent from the UI toolkit.

## Why not Tauri 2

As of 2026-09, current Tauri 2 dropped official Windows 7 support because the current WebView2 installer no longer runs there. Microsoft also no longer supports current WebView2 on Windows 7.

INFO Desktop therefore avoids making an unsupported frozen browser runtime a core dependency.

## Build model

Target:
- Windows 10/11 x64: Setup + Portable;
- Windows 7 SP1 x64: separate compatibility Portable;
- macOS arm64;
- macOS x86_64.

The exact Python/package pins for each target will be fixed only after CI builds pass.
