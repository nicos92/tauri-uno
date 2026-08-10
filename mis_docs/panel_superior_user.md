# Panel superior de usuario (TopBar)

## Objetivo

Mostrar de forma persistente y visible qué sesión está usando la aplicación,
independientemente de la página activa o del estado del sidebar (abierto/colapsado).

## Posición y layout

- Barra horizontal ubicada sobre el área de contenido, a la derecha del sidebar
  (no cubre el sidebar). Al colapsar el sidebar (60px), la barra se expande.
- Estructura: `sidebar | header + main` dentro de `.main-content`.

## Contenido

- Avatar circular con las iniciales del username (derivadas en frontend, sin
  cambios en backend).
- Username del usuario autenticado (`authStore.user.username`).
- Título de la página/módulo actual (derivado de la ruta activa, con mapa
  `route.name -> etiqueta`).
- Botón de logout (icono), que reemplaza al logout del footer del sidebar.

## Acciones

- Mover el logout al top bar. El footer del sidebar queda solo con el botón de
  colapso; se elimina el bloque `user-info` (fuente única de identidad = TopBar).

## Diseño

- Colores consistentes con el tema actual (`data-theme` light/dark).
- Username con `text-overflow: ellipsis` y `white-space: nowrap` (ancho máx fijo)
  para ventanas angostas.
- Altura fija (~56px), borde inferior sutil.

## Fuera de alcance

- Refresco en vivo de sesión/permisos.
- Edición de perfil/cambio de contraseña desde el panel.
- Avatares con foto o nombre real (la entidad User no lo tiene).

## Tareas de implementación

1. Crear `src/presentation/components/TopBar.vue` con avatar-iniciales, username,
   título de página (mapa `route.name -> label`) y botón logout.
2. Integrar `<TopBar />` en `MainLayout.vue`: `.main-content` pasa a
   `flex-direction: column` con header fijo y contenedor del `router-view` con scroll.
3. Eliminar del `sidebar-footer` el bloque `user-info` y el `logout-btn`, dejando
   solo el toggle de colapso; mover `handleLogout` al TopBar.

## Verificación

- `pnpm build` (vue-tsc + vite).
- Probar colapso del sidebar, navegación entre módulos, dark/light y logout.
