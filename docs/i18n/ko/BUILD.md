# INFO Desktop — 빌드 및 플랫폼

현재 승인된 소스 체크포인트: main의 0.1.2.

지원 대상: Windows 10/11 x64, macOS arm64, macOS x86_64, Linux x64. Windows 7은 필수가 아닙니다.

```bash
npm install
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

CI artifact does not by itself prove platform support; a real launch/smoke test is required.

## 콘솔 모드

콘솔은 Tauri와 창의 시스템 라이브러리 없이 빌드됩니다:

```bash
cd src-tauri
cargo build --release -p info-desktop-cli
cargo test --workspace --lib
```

GitHub Actions에서 콘솔은 창과 함께 Windows, Linux, macOS(ARM64 및 Intel)용으로 빌드되며, 릴리스 워크플로 `release-0.1.5.yml`은 이를 체크섬이 있는 별도 파일로 패키징합니다. 릴리스는 소유자의 명시적인 지시가 있을 때만 합니다.
