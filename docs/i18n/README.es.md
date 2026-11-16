# INFO Desktop

**INFO Desktop 0.1.6 estable — Windows / Linux / macOS:** [Descargar](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.6). Novedad: formulario de primera configuración. Versión estable anterior: [0.1.5](https://github.com/RomanZavadaM/INFO-Desktop/releases/tag/v0.1.5).

## Qué hace

- se conecta a un servidor INFO real mediante `info-desktop-protocol/1`, registro único del dispositivo;
- el secreto del dispositivo se guarda solo en el almacén seguro del sistema operativo;
- canales, cola completa, detalle de materiales, fuentes, traducciones y medios en caché;
- edita título, texto y canal; guarda, omite, aplaza y publica;
- creación manual de materiales, vista previa al estilo de Telegram;
- ajustes mediante Excel (XLSX): exportación, comprobación sin aplicar, prueba «qué cambia en 48 h», aplicación confirmada, reversión;
- temas: edición de palabras clave y comprobación con un texto de ejemplo;
- **formulario de primera configuración** (desde 0.1.6), modo consola, siete idiomas de interfaz.

## Formulario de primera configuración

El botón «Primera configuración» de la barra superior funciona incluso sin conexión al servidor. El formulario pide el nombre de la redacción, el título del bot, los valores de acceso y los canales; bajo cada campo hay una explicación y un ejemplo, y el formato (token del bot, identificadores, dirección del canal `@nombre`, enlaces t.me, sitio, zona horaria) se comprueba al instante. El resultado es el archivo `info-first-setup.xlsx` en su propio ordenador; **no se envía nada al servidor**. Coloque el archivo en el servidor (por ejemplo `/var/lib/info/inbox/`) y cárguelo con la opción 1 de `info menu`; bórrelo después, porque contiene valores de acceso. Requiere el servidor INFO `20261116-1300` o posterior. Detalles: [FIRST_SETUP_FORM](../FIRST_SETUP_FORM.md).

## Modo consola

El programa `info-desktop-cli` realiza las mismas acciones que la ventana: cola, «por qué está en este canal», crear y editar, publicar, ajustes XLSX (descarga, comprobación, prueba, aplicación, reversión) y temas. Funciona de dos formas: a través de la ventana abierta (casilla en la aplicación, solo en este equipo, la clave no se transmite) o de forma independiente (el perfil de conexión lo escribe la ventana, la clave permanece en el almacén del sistema). La confirmación `--yes` solo se exige para aplicar y revertir ajustes. Manual: [CONSOLE_MANUAL](es/CONSOLE_MANUAL.md).

## Qué se ha verificado

El 20 de octubre de 2026 Desktop se conectó a un servidor INFO real `20261020-2040`: se creó un material real mediante Desktop y se confirmó en la cola editorial y en el bot editorial de Telegram (Desktop → API → base de datos → cola). No se realizó ninguna publicación pública en Telegram. Los comandos de lectura de la consola fueron verificados por el propietario en Windows. El formulario de primera configuración solo se verificó con compilaciones automáticas: aún no se ha confirmado abrir el archivo creado en Excel ni cargarlo en un servidor.

## Plataformas

Windows 10/11 x64 · macOS ARM64 · macOS Intel x86_64 · Linux x64. Windows 7 no es un objetivo obligatorio.

## Seguridad

Sin acceso directo a SQLite, sin SSH como protocolo de aplicación, sin secretos de producción en el repositorio. Las credenciales del dispositivo tienen alcance limitado; la pasarela de INFO sigue accesible solo por VPN.

## Tecnología y compilación

Tauri 2 · TypeScript · React · Rust · reqwest/rustls.

```bash
npm install
npm run check
npm run test:protocol
npm run build
npm run tauri build
```

Véase [Compilación](../BUILD.md), [Manual de usuario](../guides/USER_MANUAL.md), [Protocolo](../protocol/INFO_DESKTOP_PROTOCOL.md), [Registro](../protocol/INFO_DESKTOP_REGISTRATION.md), [Índice de versiones](../releases/RELEASE_INDEX.md).

## Estado de la versión

La versión estable actual es **v0.1.6**. Los paquetes para Windows, Linux y macOS están publicados en GitHub Releases.

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved. Proprietary software.**
