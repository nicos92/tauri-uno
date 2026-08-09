# Guía: Cambiar el icono de la aplicación

Fecha: 2026-08-08

## Resumen

Tauri embebe el icono en el `.exe`/`.app`/`.deb` en **tiempo de compilación**, no en tiempo de ejecución. Para cambiar el icono hay que: (1) generar todos los formatos con la CLI de Tauri a partir de un **SVG cuadrado**, y (2) **forzar la recompilación** del binario porque Cargo no detecta por sí solo los cambios en `src-tauri/icons/`.

## Paso 1: Crear el SVG fuente cuadrado

El comando `tauri icon` exige una imagen **cuadrada** (PNG o SVG con transparencia).

- `public/lightBulb.svg` (usado como fondo en `src/presentation/pages/LoginPage.vue`) es 50×48 (no cuadrado) → no se puede usar directo.
- Solución aplicada: `public/icon.svg` es un lienzo cuadrado `viewBox="0 0 60 60"` (1024×1024) con la bombilla envuelta en `<g transform="translate(5.95 5.99)">` para centrarla (bbox real ~26.7×42, llena ~70%).

Para otro arte: calcular el bbox del contenido y centrarlo dentro de un lienzo cuadrado con `transform` en un `<g>`.

## Paso 2: Generar todos los iconos

```bash
pnpm tauri icon public/icon.svg
```

- `tauri-cli 2.10.1` acepta SVG y regenera **todos** los formatos en `src-tauri/icons/`:
  - Desktop: `32x32.png`, `64x64.png`, `128x128.png`, `128x128@2x.png`, `icon.png`, `icon.ico`, `icon.icns`
  - Windows Store: `StoreLogo.png`, `Square30x30Logo.png` … `Square310x310Logo.png`
  - iOS: `AppIcon-*.png`
  - Android: `mipmap-*/ic_launcher*.png`
- La lista de `tauri.conf.json` (`bundle.icon`) ya referencia `32x32.png`, `128x128.png`, `128x128@2x.png`, `icon.icns`, `icon.ico` → **no hace falta tocar la config** (los nombres que genera la CLI coinciden).
- Si se quiere otro color de fondo en iOS: `--ios-color <color>`.

## Paso 3: Forzar la recompilación (clave)

`tauri-build` NO registra `src-tauri/icons/` como `rerun-if-changed`, así que `cargo build`/`pnpm tauri dev` **no relinkean** el `.exe` al cambiar solo los iconos: el binario conserva el icono viejo (se nota porque el timestamp del `.exe` no cambia).

Forzar rebuild del paquete:

```bash
cd src-tauri
cargo clean -p Calise-App
cargo build          # o directamente: pnpm tauri dev
```

- `cargo clean -p Calise-App` solo limpia los artefactos de nuestra crate (no las dependencias), así que el rebuild es rápido (~25 s).
- Después del primer rebuild ya no es necesario limpiar: la próxima vez que cambie el icono alcanza con `cargo clean -p Calise-App` + `cargo build`.

## Verificación

- El `.exe` debe tener un timestamp posterior a la generación de `icons/icon.ico`.
- Chequear el icono embebido (PowerShell):

```powershell
Add-Type -AssemblyName System.Drawing
$icon = [System.Drawing.Icon]::ExtractAssociatedIcon("src-tauri\target\debug\Calise-App.exe")
$bmp = $icon.ToBitmap()
$bmp.GetPixel(16,16)   # centro: debe dar el color del arte (ej. bombilla: R252 G213 B63 A255)
$bmp.GetPixel(2,2)     # esquina: debe dar transparente (A0)
```

- Si el taskbar/Explorer sigue mostrando el icono viejo pese al rebuild, es caché de iconos de Windows:

```powershell
Stop-Process -Name explorer -Force   # se reinicia solo; o reiniciar el sistema
```
