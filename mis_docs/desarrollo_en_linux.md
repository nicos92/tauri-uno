# Desarrollo del proyecto en Linux

Guía para seguir desarrollando Calise-App en Linux (proyecto creado originalmente en Windows).

---

## 1. Requisitos del sistema

Tauri 2 necesita librerías nativas de GTK/WebKit que no vienen instaladas por defecto. Sin ellas, `cargo` falla al compilar crates como `gdk-sys` con errores de `pkg-config` (ej. `The system library gdk-3.0 required by crate gdk-sys was not found`).

### Fedora (dnf)

```bash
sudo dnf install -y webkit2gtk4.1-devel openssl-devel curl wget file \
  libappindicator-gtk3-devel librsvg2-devel patchelf
```

Esto instala por dependencia `gtk3-devel` (que provee `gdk-3.0.pc`), `libsoup3-devel` y `javascriptcoregtk4.1-devel`.

> En Ubuntu/Debian el equivalente sería: `sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev`

### Herramientas de build

Verificar que `gcc`, `make` y `pkg-config` estén disponibles:

```bash
command -v gcc make pkg-config
```

---

## 2. Verificar que las librerías están instaladas

```bash
pkg-config --exists gdk-3.0 && echo "gdk OK"
pkg-config --exists webkit2gtk-4.1 && echo "webkit OK"
```

Si imprime `OK` en ambos, la compilación de las dependencias nativas debería funcionar.

---

## 3. Comandos para desarrollar

### Desarrollo (frontend + backend)

```bash
pnpm tauri dev
```

### Sólo frontend (Vite, sin ventana Tauri)

```bash
pnpm dev
```

### Build de producción

```bash
pnpm tauri build
```

### Verificación de código

```bash
pnpm build                         # vue-tsc --noEmit (typecheck) + vite build
cd src-tauri && cargo check        # verificar Rust sin compilar
cd src-tauri && cargo clippy       # linter de Rust
```

---

## 4. Troubleshooting

### Error de `gdk-sys` / `webkit2gtk` en la primera compilación

Es el error del paso 1: faltan librerías del sistema. Instalar con dnf y volver a correr `pnpm tauri dev`. Cargo reintenta el build script fallido automáticamente.

### Builds fallidos que no se recuperan

Si cargo sigue quejándose de un crate nativo después de instalar las librerías:

```bash
rm -rf src-tauri/target/debug/build
pnpm tauri dev
```

### PKG_CONFIG_PATH

Con los paquetes `-devel` instalados por dnf, los archivos `.pc` están en `/usr/lib64/pkgconfig`, que ya está en la ruta por defecto de pkg-config. **No** es necesario setear `PKG_CONFIG_PATH` a mano salvo que se use un prefijo custom.

### Archivosystem case-sensitive (diferencia con Windows)

En Windows/macOS el filesystem no distingue mayúsculas; en Linux sí. Los nombres de archivo Rust en `src-tauri/src` deben coincidir **exactamente** con el módulo declarado en el `mod.rs` correspondiente (ej. `categoria_repository.rs` para `pub mod categoria_repository;`). Un desajuste de mayúsculas compila en Windows pero rompe la compilación en Linux.

Si se renombra un archivo cambiando sólo mayúsculas, usar `git mv` en dos pasos (nombre temporal → destino) porque el repo está configurado con `core.ignorecase=true`.

### Dependencias instaladas con `npm install` / `pnpm install`

Es normal que falten algunos paquetes al pasar de Windows a Linux; el `node_modules` de Windows no se reutiliza. Correr:

```bash
pnpm install
```

antes del primer `pnpm tauri dev`.
