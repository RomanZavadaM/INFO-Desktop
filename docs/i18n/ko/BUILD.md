# INFO Desktop — 빌드 및 플랫폼

현재 승인된 소스 체크포인트: main의 0.1.1.

지원 대상: Windows 10/11 x64, macOS arm64, macOS x86_64, Linux x64. Windows 7은 필수가 아닙니다.

```bash
npm ci
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

CI artifact does not by itself prove platform support; a real launch/smoke test is required.
