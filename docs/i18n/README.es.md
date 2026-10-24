# INFO Desktop

> ⬇️ **Descargar INFO Desktop 0.1.3**: [**Windows EXE / MSI · Linux AppImage / DEB · macOS Intel / Apple Silicon**](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.3)

**INFO Desktop 0.1.3 ya está publicado.** [Descargar v0.1.3](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.3) para Windows, Linux y macOS Intel/Apple Silicon. Incluye configuración XLSX y temas; todavía falta la aceptación integral en Desktop.

## Funciones

- conexión a un servidor INFO real mediante `info-desktop-protocol/1`;
- archivo de registro de dispositivo de un solo uso;
- secreto reutilizable guardado solo en el almacén seguro del sistema operativo;
- canales, cola pendiente completa, detalles, fuentes y medios en caché;
- edición de título, texto y canal de destino;
- guardar, omitir y aplazar;
- creación manual de nuevos materiales;
- vista previa estilo Telegram;
- publicación con permisos por scope;
- siete idiomas de interfaz.

## Aceptación en producción

El 20 de octubre de 2026 INFO Desktop se conectó a LIVE INFO `20261020-2040`. Se creó un material real de Lviv desde Desktop y apareció en la cola editorial y en el bot editorial de Telegram. Se verificó Desktop → API → base de datos → cola editorial. No se realizó ningún SEND público de Telegram durante esta aceptación.

## Plataformas

Windows 10/11 x64 · macOS ARM64 · macOS Intel x86_64 · Linux x64. Windows 7 no es un objetivo obligatorio.

## Seguridad

Sin acceso directo a SQLite, sin SSH como protocolo de aplicación y sin secretos de producción en el repositorio. Las credenciales usan scopes; el gateway actual sigue siendo solo por VPN.

## Tecnología y compilación

Tauri 2 · TypeScript · React · Rust · reqwest/rustls.

```bash
npm install
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

Consulte [Build](../BUILD.md), [Manual](../guides/USER_MANUAL.md), [Protocolo](../protocol/INFO_DESKTOP_PROTOCOL.md), [Registro](../protocol/INFO_DESKTOP_REGISTRATION.md), [Índice de versiones](../releases/RELEASE_INDEX.md).

## Estado de la versión

El checkpoint `0.1.2` está aceptado en `main`. See release status above.

**Copyright © 2026 Roman Zavada (Роман Завада). Todos los derechos reservados. Software propietario.**
