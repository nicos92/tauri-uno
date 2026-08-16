# Plan: Refactorización del frontend hacia Clean Architecture

## Objetivo

Reestructurar el frontend (Vue 3 + TypeScript + Pinia) para mejorar su
arquitectura, mantenibilidad, legibilidad y testabilidad, siguiendo los
principios SOLID y Clean Architecture, sin reescribir el sistema completo y de
forma incremental.

El criterio de éxito no es "tener Clean Architecture" por sí misma, sino que un
desarrollador pueda leer el código y responder rápidamente:

```text
¿Qué quiere hacer el usuario?
        ↓
¿Qué caso de uso se ejecuta?
        ↓
¿Qué reglas de negocio intervienen?
        ↓
¿Qué datos necesita?
        ↓
¿Qué infraestructura utiliza?
```

## Decisiones rectoras

- **Incremental, no reescritura**: una etapa a la vez, verificando que todo
  sigue funcionando antes de continuar.
- **Proporcional a la complejidad real**: no se agregan interfaces ni
  abstracciones "por cumplir SOLID". Cada abstracción debe resolver un problema
  real de acoplamiento, testabilidad o evolución.
- **No tocar código que funciona por preferencias de estilo**.
- **Orden de prioridad**: simple → cohesivo → desacoplado → testeable →
  extensible.
- **Backend (Rust) queda intacto**: este plan toca exclusivamente `src/`.
- **No se agregan permisos**: la sincronización de permisos (Rust/TS/seed) no
  cambia en ninguna etapa.

---

## 1. Estado actual

### Stack

Tauri 2 + Vue 3 (`<script setup lang="ts">`) + TypeScript strict + Pinia +
Vue Router. Sin tests ni lint; `pnpm build` = `vue-tsc --noEmit` + `vite build`.

### Estructura actual (~13.000 líneas TS/Vue)

```text
src/
├── domain/            # entities (types.ts 464 líneas, permissions.ts, appError.ts) + interfaces/userRepository.ts
├── application/       # 7 usecases SOLO del módulo usuarios
├── infrastructure/    # api/ con 14 repositorios + errorHandler.ts
└── presentation/
    ├── pages/         # 18 páginas (NuevaVentaPage 1.381 líneas, UsersPage 702, ...)
    ├── stores/        # index.ts con 14 stores en 1.231 líneas + themeStore.ts
    ├── composables/   # usePermissions (248), useToasts, useConfirm
    ├── components/    # TopBar, ConfirmDialog, Toasts (los únicos 4)
    ├── layouts/       # MainLayout
    └── router/        # guard con loadFromStorage + meta.permission
```

### Lo que ya está bien hecho

- Tipos de dominio centralizados en un solo lugar (`domain/entities/types.ts`).
- El módulo **usuarios** tiene la arquitectura completa: interfaz
  `IUserRepository` → `UserApiRepository` → usecases → stores.
- `errorHandler.toErrorMessage` es la única pieza de infraestructura
  compartida y bien factorizada.
- Routing con lazy-load y guard de permisos por `meta.permission`.

---

## 2. Problemas encontrados

1. **`stores/index.ts` de 1.231 líneas** concentra los 14 stores (auth, users,
   permissions, proveedores, categorias, subCategorias, articulos, stock,
   audit, ventas, tiposVenta, cierres, clientes, home, dolar, presupuestos).
2. **Páginas "mega componente"**: `NuevaVentaPage.vue` (1.381 líneas) mezcla
   orquestación de datos, carrito, conversión presupuesto→venta, modal de
   cliente, impresión PDF y 4 stores; `UsersPage.vue` (702 líneas) junta tabla
   + 4 modales (crear, editar, permisos, contraseña).
3. **Patrón CRUD duplicado 7×**: Proveedores, Categorías, SubCategorías,
   Clientes, TiposVenta, Articulos y Stock repiten el mismo esqueleto (header +
   tabla con permisos + modal crear + modal editar + confirmación de borrado +
   búsqueda), reescrito a mano en cada página.
4. **`getCurrentUserId()` duplicado 14 veces** (una por repositorio), leyendo
   `sessionStorage["currentUser"]`.
5. **Solo 1 de 14 repositorios implementa una interfaz**
   (`UserApiRepository implements IUserRepository`). Los otros 13 son clases
   concretas sin contrato → no sustituibles ni fakeables.
6. **`LoginResponse` definido dos veces** (domain/interfaces y
   infrastructure/api).
7. **Tipos muertos**: `PresupuestoFilter` y `GetPresupuestosRequest` se definen
   pero `presupuestoRepository` usa un objeto literal.
8. **Inconsistencias de convención**: archivo `CategoriaRepository.ts` en
   PascalCase (el resto camelCase); métodos en español en `clienteRepository`
   (`crearCliente`) vs inglés en el resto; `tipoVentaRepository.updateTipoVenta`
   envía `id` y `request` de más; `getStockByArticulo` devuelve `Stock | null`
   mientras los getters hermanos no.
9. **~150–200 líneas de `<style scoped>` casi idénticas por página**
   (`.btn-primary`/`.modal-overlay`/`.table-wrapper`/…) y colores hardcodeados
   `#3F2281`, `#5568d3`, `#e53e3e` que ignoran las CSS vars `--color-*`.
10. **Bug latente**: en 5 páginas el estado vacío se muestra mientras carga (no
    está protegido con `!loading`).
11. **Paginación duplicada al pie de la letra** entre CierresPage y
    AuditoriaPage (lógica + CSS).
12. **Reglas de negocio en la vista**: `calcularPrecioVenta` en el store
    (presentación); `isDefaultClient` ("Consumidor"/"Final") en ClientesPage;
    semántica de fechas de filtro en AuditoriaPage; join sub-categoría O(n·m)
    en SubCategoriasPage.
13. **Acoplamiento store→store**: `useVentasStore.createVenta/anularVenta`
    llaman `useStockStore().fetchStock()`.
14. **Flujo de negocio en la página**: la conversión presupuesto→venta (crear
    venta + marcar `convertido`) está orquestada dentro de
    `NuevaVentaPage.handleCreate`.
15. **Desvío de documentación**: AGENTS.md §9.10 dice `localStorage`, pero el
    código usa `sessionStorage`.
16. **Barrels incompletos**: `infrastructure/api/index.ts` solo exporta
    `userRepository`; los stores importan los otros 13 por ruta completa.

---

## 3. Violaciones SOLID

- **S — Single Responsibility**: `stores/index.ts` (14 stores) y
  `NuevaVentaPage.vue` (orquestación + UI + carrito + impresión + modal
  cliente). También `usePermissions.ts`, lista a mano de 40+ helpers.
- **O — Open/Closed**: agregar una entidad exige tocar ~6 archivos (repo,
  store, página, usePermissions, router, menú del layout) copiando el patrón;
  agregar un permiso obliga a editar `usePermissions.ts`.
- **L — Liskov**: no aplica casi porque no hay interfaces que sustituir (solo
  `IUserRepository`).
- **I — Interface Segregation**: `IUserRepository` es una interfaz grande de 10
  métodos; los stores de usuarios/permisos la consumen completa aunque cada uno
  solo use una parte.
- **D — Dependency Inversion**: los stores dependen de clases concretas
  instanciadas en el módulo (`const proveedorRepository = new
  ProveedorApiRepository()`); no se puede inyectar un fake. Además
  `presentation/stores` importan `infrastructure/api` directamente sin pasar
  por `application/`.

---

## 4. Problemas de Clean Architecture

```text
Presentation (stores) ──────→ Infrastructure (14 repos concretos)   ← flujo real
Presentation (stores) ──────→ Application → Domain → Infrastructure  ← solo usuarios
```

- Los stores de **proveedores, categorias, articulos, stock, clientes, ventas,
  presupuestos, cierres, dolar, auditoria, home, tiposVenta** saltan la capa de
  aplicación: `store → repo concreto`.
- **`calcularPrecioVenta` (regla de negocio `costo * (1 + ganancia/100)`) vive
  en el store** (presentación) en vez de dominio/aplicación.
- `sessionStorage` se lee desde la infraestructura (14 repos) y se escribe desde
  presentación → conocimiento de almacenamiento duplicado y en la capa
  incorrecta.
- `NuevaVentaPage` conoce demasiados detalles de flujo de negocio (estados
  terminales de presupuesto, conversión) que deberían estar en un caso de uso.
- No hay inyección de dependencias: los repos se instancian en el store, no se
  reciben.

---

## 5. Problemas de testabilidad

| Pieza | Problema |
|---|---|
| Stores (no-users) | Dependen de clases concretas instanciadas en el módulo; para testear hay que mockear `invoke` de Tauri. Sin interfaz → sin `FakeProveedorRepository`. |
| Páginas | Lógica embebida en `.vue` no importable; dependen de Pinia, router y `sessionStorage`. |
| `usePermissions` | Depende de `useAuthStore` internamente. |
| `NuevaVentaPage` | La lógica del carrito y conversión no es extraíble sin montar el componente. |
| Repos | `getCurrentUserId()` lee `sessionStorage` directamente. |
| Reglas de negocio | `calcularPrecioVenta`, `isDefaultClient`, filtros de fecha están en la UI → no se pueden testear sin DOM. |

---

## 6. Componentes que deberían dividirse

- **NuevaVentaPage (1.381)** → contenedor delgado + `useCart` composable +
  `ArticuloSearch`, `CartTable`, `TotalsSummary`, `ClienteSelector`,
  `NuevoClienteModal`, `PresupuestoPrintArea`.
- **UsersPage (702)** → `UsersTable`, `UserFormModal`, `PermissionsModal`,
  `PasswordChangeModal`.
- **Páginas CRUD (7)** → componentes compartidos: `DataTable`,
  `EntityFormModal`, `ConfirmButton`, `PaginationBar`, `PageHeader`,
  `SearchBar`.
- **CierresPage / AuditoriaPage** → `usePagination` composable +
  `PaginationBar` (idénticos hoy).
- **HomePage** → `StatCard` (6 tarjetas repetidas con SVG inline).
- **ArticulosPage/StockPage** → los computed de join/enriquecimiento pasan a
  selectors del store o a `utils`.

---

## 7. Casos de uso

Existentes (solo usuarios): `Login`, `CreateUser`, `GetAllUsers`, `UpdateUser`,
`DeleteUser`, `ChangePassword`, `ManagePermissions`.

Faltantes (operaciones atómicas de negocio):

- `RegistrarVenta` (decrementa stock + crea venta).
- `AnularVenta` (restaura stock).
- `ConvertirPresupuestoEnVenta` (crear venta + marcar `convertido`).
- `CrearPresupuesto`, `CambiarEstadoPresupuesto`, `ObtenerPresupuestos`.
- `CrearClienteRapido`.
- `ObtenerPrecioVenta`.
- `CerrarDia`, `ReabrirDia`.
- CRUD restantes: `CrearProveedor`, `ObtenerProveedores`, `ActualizarProveedor`,
  `EliminarProveedor`, y equivalentes para categorías, sub-categorías,
  artículos, stock, clientes y tipos de venta.

---

## 8. Arquitectura propuesta

Mantener las 4 capas existentes; el cambio es **completar las capas faltantes y
estandarizar**.

```text
src/
├── domain/
│   ├── entities/            (ya correcto; sumar constantes de negocio)
│   └── interfaces/          + IProveedorRepository, IArticuloRepository,
│                              IVentaRepository, IClienteRepository,
│                              IPresupuestoRepository, ... (1 por feature)
├── application/usecases/    + usecases de cada feature (patrón de usuarios)
├── infrastructure/
│   ├── api/                 repos que implementan las interfaces
│   │                        + helper getCurrentUserId compartido
│   └── utils/               sessionStore.ts, apiInvoke.ts
├── presentation/
│   ├── stores/              UN archivo por store (proveedoresStore.ts,
│   │                        ventasStore.ts, ...)
│   ├── composables/         usePagination, useCart, useEntityCrud,
│   │                        usePermissions (generado desde PERMISSIONS)
│   ├── components/          ui/ (Modal, DataTable, PaginationBar,
│   │                        ConfirmButton, SearchBar) + <feature>/ (por feature)
│   ├── pages/               contenedores delgados
│   └── utils/               date.ts, format.ts, cliente.ts (clienteLabel)
└── assets/styles/           tokens de botones/modales/tablas
                             (elimina el CSS duplicado)
```

**Decisión explícita**: no se introduce una abstracción por entidad de más. Los
repos solo obtienen interfaz donde aporte testabilidad o sustitución real. El
valor principal está en: separar stores, extraer composables, componentes
compartidos, mover reglas de negocio a dominio y unificar el acceso a
`sessionStorage`/`invoke`.

---

## 9. Plan de refactorización por etapas

De menor a mayor riesgo. Cada etapa indica: qué se modifica, por qué, qué
principio mejora, qué dependencias se eliminan, cómo mejora la testabilidad y
qué riesgo tiene.

### Etapa 1 — Extracciones puras, cero cambio de comportamiento

**Qué se modifica:**

- Helper compartido `getCurrentUserId()` en `infrastructure/utils/currentUser.ts`
  (y opcionalmente `sessionStore.ts`); los 14 repos lo usan.
- Dividir `stores/index.ts` en un archivo por store (mismo código, solo mover):
  `stores/authStore.ts`, `usersStore.ts`, `permissionsStore.ts`,
  `proveedoresStore.ts`, `categoriasStore.ts`, `subCategoriasStore.ts`,
  `articulosStore.ts`, `stockStore.ts`, `ventasStore.ts`, `tiposVentaStore.ts`,
  `cierresStore.ts`, `clientesStore.ts`, `homeStore.ts`, `dolarStore.ts`,
  `presupuestosStore.ts`. El `index.ts` queda como barrel.
- CSS: tokens globales para `.btn-*`, `.modal-*`, `.table-wrapper`, `.error-*`;
  reemplazar hex hardcodeados (`#3F2281`, `#5568d3`, `#e53e3e`) por `--color-*`.
- `utils/date.ts` (`todayLocal`, `formatTimestamp`) y centralizar
  `clienteLabel`.

**Por qué:** ataca los problemas 1, 4, 9, 16 y reduce la fricción de navegar el
código sin cambiar comportamiento.

**Principios:** S, DRY.

**Dependencias que se eliminan:** importaciones por ruta profunda a los repos;
duplicación de `getCurrentUserId` y de CSS.

**Testabilidad:** cada store pasa a ser importable por separado.

**Riesgo:** bajo (solo mover). Verificar con `pnpm build` después de cada
movimiento de archivo.

### Etapa 2 — Composables de presentación

**Qué se modifica:**

- `usePagination` composable + `PaginationBar` (CierresPage y AuditoriaPage,
  que hoy son idénticos).
- `useCart` (NuevaVentaPage) → la página baja de 1.381 a ~500 líneas.

**Por qué:** ataca los problemas 2 y 11.

**Principios:** S, DRY.

**Dependencias que se eliminan:** lógica de paginación y de carrito duplicada.

**Testabilidad:** `useCart` y `usePagination` pasan a ser unit-testables sin
DOM.

**Riesgo:** medio-bajo.

### Etapa 3 — Componentes UI compartidos

**Qué se modifica:**

- Componentes `ui/`: `Modal`, `EntityFormModal`, `DataTable`, `ConfirmButton`,
  `SearchBar`, `StatCard`, `PageHeader`.
- Refactorizar las 7 páginas CRUD sobre ellos, **una a la vez**
  (Proveedores → Categorías → SubCategorías → TiposVenta → Clientes →
  Articulos → Stock).

**Por qué:** ataca los problemas 3, 6, 9, 10 (de paso se corrige el
empty-state-while-loading).

**Principios:** S, O (una entidad nueva ya no duplica markup).

**Dependencias que se eliminan:** ~1.000+ líneas de CSS y markup duplicados.

**Testabilidad:** componentes con props/emits tipados, aislables en tests de
componentes.

**Riesgo:** medio (toca las páginas más usadas; probar pantalla por pantalla).

### Etapa 4 — Reglas de negocio a dominio/aplicación

**Qué se modifica:**

- `calcularPrecioVenta` → dominio (constante/helper puro); los llamadores pasan
  a usarla. Validar contra el comando backend `get_precio_venta` que ya existe.
- `isDefaultClient` ("Consumidor"/"Final") → constante de dominio o helper.
- Semántica de fechas de filtro (AuditoriaPage `buildFilters`) → helper o al
  repository.
- Joins de SubCategorias/Articulos (O(n·m)) → selectors del store o lookup con
  `Map`.

**Por qué:** ataca los problemas 12 y el flujo de capas.

**Principios:** D, Clean Architecture.

**Dependencias que se eliminan:** presentación dependiendo de reglas de
negocio.

**Testabilidad:** reglas puras testables sin Vue.

**Riesgo:** medio (validar contra backend para no romper cálculos).

### Etapa 5 — Contratos de repositorio + inyección

**Qué se modifica:**

- Interfaces `I*Repository` por feature; los repos las implementan.
- Los stores reciben el repo (o se registra en un módulo `infrastructure/di`),
  en lugar de instanciarlo en el módulo.
- Limpieza: eliminar `LoginResponse` duplicado, tipos muertos
  (`PresupuestoFilter`, `GetPresupuestosRequest`), `id` redundante de
  tipoVenta, unificar el contrato null de `getStockByArticulo`.

**Por qué:** ataca los problemas 5, 6, 7, 8 y el D.

**Principios:** D, L, I.

**Dependencias que se eliminan:** acoplamiento store → clase concreta.

**Testabilidad:** permite `FakeProveedorRepository` en tests de stores/usecases.

**Riesgo:** medio-alto (toca todos los stores).

### Etapa 6 — Casos de uso para el resto de features

**Qué se modifica:**

- `application/usecases/ventas.ts`, `presupuestos.ts`, `clientes.ts`, etc.; los
  stores pasan por ellos (patrón del módulo usuarios).
- Extraer `ConvertirPresupuestoEnVenta` fuera de `NuevaVentaPage`.

**Por qué:** ataca los problemas 13, 14 y completa la capa de aplicación.

**Principios:** Clean Architecture, S.

**Dependencias que se eliminan:** orquestación de negocio dentro de la UI y
acoplamiento store→store.

**Testabilidad:** la lógica de negocio queda 100% testeable con fakes.

**Riesgo:** alto (última etapa funcional).

### Etapa 7 — Higiene final

**Qué se modifica:**

- Corregir bug de empty-state-while-loading pendiente.
- Actualizar AGENTS.md §9.10 (`localStorage` → `sessionStorage`) y los puntos de
  arquitectura que cambien.
- Renombrar `CategoriaRepository.ts` a `categoriaRepository.ts`
  (con `git mv` en dos pasos por `core.ignorecase=true`).
- Barrels completos: `infrastructure/api/index.ts` exporta todos los repos.
- `usePermissions` generado desde `PERMISSIONS` (elimina los 40+ helpers a mano)
  o al menos sincronizado.

**Por qué:** cierra inconsistencias y deja la documentación alineada con el
código.

**Riesgo:** bajo.

---

## 10. Ejemplo concreto (patrón a replicar)

Tomando **Proveedores** (el CRUD más simple, sirve de patrón para el resto):

```text
src/
├── domain/interfaces/proveedorRepository.ts
├── application/usecases/proveedores.ts
├── infrastructure/api/proveedorRepository.ts        (implementa la interfaz)
├── infrastructure/utils/currentUser.ts              (getCurrentUserId único)
└── presentation/
    ├── stores/proveedoresStore.ts                   (inyecta IProveedorRepository)
    ├── composables/useEntityCrud.ts
    ├── components/ui/Modal.vue, DataTable.vue, PaginationBar.vue, ConfirmButton.vue
    ├── components/proveedores/ProveedorFormModal.vue
    └── pages/ProveedoresPage.vue                    (contenedor delgado)
```

```ts
// domain/interfaces/proveedorRepository.ts
export interface IProveedorRepository {
  getAllProveedores(): Promise<Proveedor[]>;
  createProveedor(r: CreateProveedorRequest): Promise<Proveedor>;
  updateProveedor(r: UpdateProveedorRequest): Promise<Proveedor>;
  deleteProveedor(id: number): Promise<void>;
}
```

```ts
// application/usecases/proveedores.ts
export class CrearProveedorUseCase {
  constructor(private repo: IProveedorRepository) {}
  execute(r: CreateProveedorRequest) { return this.repo.createProveedor(r); }
}
```

```ts
// presentation/stores/proveedoresStore.ts (fragmento)
export const useProveedoresStore = defineStore("proveedores", () => {
  const proveedores = ref<Proveedor[]>([]);
  const crear = async (r: CreateProveedorRequest) => {
    const creado = await new CrearProveedorUseCase(repo).execute(r);
    proveedores.value.push(creado);
  };
  return { proveedores, crear };
});
```

`ProveedorFormModal` recibe `modelValue` y emite `submit`; `ProveedoresPage`
queda como composición: header + `DataTable` + `ProveedorFormModal` +
`ConfirmButton`, sin lógica de negocio. El mismo esqueleto se replica luego a
categorías, artículos, stock, etc.

---

## Criterio de aceptación por etapa

Cada etapa se considera completa cuando:

1. `pnpm build` (typecheck + build) pasa sin errores.
2. La funcionalidad afectada se prueba manualmente en `pnpm tauri dev`.
3. No quedan imports huérfanos (verificar con `vue-tsc --noEmit`).
4. El comportamiento no cambia (solo estructura), salvo correcciones de bugs
   explícitas y justificadas (ej. empty-state-while-loading).

## Orden sugerido de ejecución

Recomendado: **Etapa 1** primero (bajo riesgo) y **Etapa 3** después (mayor
impacto visual), dejando la **Etapa 6** al final por su riesgo.
