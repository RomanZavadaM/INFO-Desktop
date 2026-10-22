# INFO Desktop 0.1.2

Fecha: 2026-10-22

Novedades:
- botón «¿Por qué en este canal?» en la ficha del elemento: las decisiones de todos los canales para el evento y el árbol de condiciones XLSX del canal elegido (requiere el servidor INFO `20261022-2010` o posterior);
- la versión de la aplicación ahora se muestra en la cabecera;
- los archivos de sumas de verificación (`SHA256SUMS-*.txt`) usan los mismos nombres que los archivos de la versión (sin espacios), por lo que `sha256sum -c` funciona por nombre;
- el servidor solo sirve vistas previas de imágenes pequeñas (los vídeos y archivos grandes se abren mediante el enlace de la fuente).

Sin cambios: protocolo `info-desktop-protocol/1`, registro, seguridad. La publicación desde Desktop no se probó en esta versión.
