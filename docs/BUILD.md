# INFO Desktop — build matrix

Build layout follows the established Taxo product pattern.

## Windows 10/11 x64
- modern Python runtime;
- portable ZIP first;
- Setup installer will be added after the first executable acceptance.

## Windows 7 SP1 x64
- separate compatibility build;
- Python 3.8.10 compatibility runtime;
- Portable ZIP;
- acceptance requires a real launch on Windows 7, not CI alone.

## macOS
- Apple Silicon / arm64;
- Intel / x86_64;
- portable application ZIPs;
- signing/notarization policy will be fixed before stable 1.0.

## Rule
A platform is not declared supported merely because CI produced an artifact. Real launch/acceptance is required before stable claims.
