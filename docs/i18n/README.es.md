# INFO Desktop

**INFO Desktop 0.1.5 estable — Windows / Linux / macOS:** [Descargar](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.5). Ciclo editorial más rápido y el nuevo modo consola (`info-desktop-cli` para Windows, Linux y macOS). Versión estable anterior: [0.1.4](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.4).




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

## Modo consola

El programa `info-desktop-cli` realiza las mismas acciones que la ventana: cola, «por qué en este canal», crear y editar, publicar, ajustes XLSX (descargar, comprobar, prueba, aplicar, revertir) y temas. Funciona de dos maneras: a través de la ventana abierta (casilla en la aplicación, solo en este equipo, la clave nunca se transmite) o de forma independiente (la ventana escribe el perfil de conexión, la clave queda en el almacén de credenciales del sistema). La confirmación `--yes` solo hace falta para aplicar y revertir ajustes. Versión 0.1.5. Verificado por el propietario en Windows con un servidor real: comandos de lectura. Manual: [CONSOLE_MANUAL](es/CONSOLE_MANUAL.md).

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

La versión estable actual es **v0.1.4**. El GitHub Release está publicado para Windows, Linux y macOS. La aceptación manual en Windows WebView2 del nuevo editor visual sigue pendiente.

**Copyright © 2026 Roman Zavada (Роман Завада). Todos los derechos reservados. Software propietario.**
