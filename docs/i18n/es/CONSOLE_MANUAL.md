# Modo consola de INFO Desktop: manual breve

Estado: en desarrollo (versión 0.1.5, sin publicar). Dos modos: a través de la ventana abierta de la aplicación y de forma independiente.

## Qué es
El programa `info-desktop-cli` hace lo que la ventana: muestra la cola, explica rutas, crea y edita elementos, publica, trabaja con los ajustes XLSX. La clave del dispositivo sale del mismo almacén seguro del sistema que la de la ventana y nunca se imprime.

## Modos
- **A través de la ventana abierta (conexión compartida).** En la ventana marque «Permitir que la consola funcione a través de esta ventana» (desactivada por defecto). Mientras la ventana esté abierta, la consola usa su conexión: VPN y clave ya están configurados y la clave nunca se transmite. La entrada es local a este equipo (127.0.0.1) y está protegida por una contraseña de sesión de un solo uso guardada en la carpeta de ajustes del usuario. La ventana muestra cada acción de la consola junto a la casilla.
- **Independiente.** No hace falta ventana: la consola se conecta sola al servidor con el perfil de conexión y la clave del almacén del sistema. La ventana escribe el perfil en cada conexión (dirección, puerto, número de dispositivo, certificado del servidor; sin claves); también puede fijarse a mano (`profile set`).
- Elección: `--mode auto` (por defecto: la ventana si está abierta y lo permite, si no independiente), `--mode app` (solo ventana), `--mode alone` (solo independiente). Un `--profile` explícito sin `--mode` significa independiente.

## Primer arranque
1. Conéctese en la ventana como siempre (en Windows con nuestro servidor debe estar activa la VPN).
2. La ventana escribe el perfil sola. Solo si hace falta a mano: `info-desktop-cli profile set --server DIRECCIÓN --port PUERTO --device-id NÚMERO --ca-file ARCHIVO`; el archivo contiene el valor `server_ca_der` del archivo de registro.
3. Compruebe la conexión: `info-desktop-cli status`.

## Comandos
`info-desktop-cli` sin argumentos muestra la ayuda completa.

| Tarea | Comando |
|---|---|
| Lista de canales | `channels` |
| Cola | `queue list [--limit N] [--cursor N] [--channel CÓDIGO] [--bucket GRUPO]` |
| Elemento con fuentes | `queue show ID` |
| Por qué está en este canal | `why ID --channel CÓDIGO [--scope route\|auto\|digest\|instant\|relevance]` |
| Crear un elemento | `create --channel CÓDIGO --title TEXTO (--text TEXTO \| --text-file ARCHIVO) [--url URL]` |
| Editar un elemento | `save ID --title TEXTO (--text TEXTO \| --text-file ARCHIVO) [--channel CÓDIGO]` |
| Omitir, aplazar, publicar | `skip ID`, `defer ID [--minutes N]`, `publish ID` |
| Ajustes XLSX | `settings status`, `settings download --out ARCHIVO`, `settings check ARCHIVO`, `settings probe [--hours N]`, `settings probe-status`, `settings apply [--without-probe] [--yes]`, `settings rollback [--yes]` |
| Temas y palabras | `themes list`, `themes test ...`, `themes save ...` |

## Cambiar ajustes
Primero `settings check` (comprueba el archivo sin aplicar), luego `settings probe` y `settings probe-status` («qué cambiará»), después `settings apply`. Sin `--yes` el comando solo muestra lo que se aplicaría. Se aplica exactamente el archivo comprobado (por huella). Sin prueba el comando se niega salvo con `--without-probe`. Revertir: `settings rollback --yes`.

Publicar (`publish`) y las acciones editoriales habituales no necesitan confirmación: a criterio y bajo responsabilidad del ejecutor; una publicación siempre puede eliminarse en el canal.

## Códigos de salida
0 éxito, 1 error de comando, 2 argumentos incorrectos, 3 sin conexión o sin clave, 4 el servidor rechazó (sin derechos, conflicto).

## Aún no hecho
Funcionamiento en un servidor sin sesión gráfica (aplazado por el propietario). Verificado en Windows el 5 de noviembre de 2026 con un servidor real (lectura): `status`, `channels`, `queue list`, `why` a través de la ventana, `status` independiente y automático. Sin verificar: comandos de escritura, XLSX y temas, macOS y Linux.
