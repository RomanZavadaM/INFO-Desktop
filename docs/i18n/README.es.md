# INFO Desktop

[Українська](../../README.md) · [English](README.en.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [한국어](README.ko.md) · [日本語](README.ja.md)

**INFO Desktop es una estación de trabajo editorial multiplataforma para servidores compatibles con INFO.**

La versión **0.1.1** es el primer punto de control aceptado e integrado en `main`.

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

El checkpoint `0.1.1` está aceptado en `main`. Los instaladores públicos de GitHub Release son el siguiente paso.

**Copyright © 2026 Roman Zavada (Роман Завада). Todos los derechos reservados. Software propietario.**
