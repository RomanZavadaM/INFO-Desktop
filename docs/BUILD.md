# INFO Desktop — build matrix

## Required targets
- Windows 10/11 x64.
- macOS Apple Silicon / arm64.
- macOS Intel / x86_64.
- Linux x64.

## Windows 7
Not part of the required matrix as of 2026-10-20. Keeping it as a hard requirement would force legacy runtime decisions into a new product. A separate legacy client can be considered later if a real need appears.

## Acceptance rule
CI artifact ≠ supported platform. Before stable documentation claims support, the application must actually start and complete the relevant smoke test on that OS.
