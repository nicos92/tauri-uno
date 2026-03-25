# AGENTS.md - Guía para Agentes de Código

Este documento proporciona instrucciones y convenciones para agentes de código que operan en este repositorio.

---

## 1. Resumen del Proyecto

- **Stack**: Tauri 2 + Vue 3 + TypeScript + Vite
- **Package Manager**: pnpm
- **Frontend**: TypeScript strict mode habilitado
- **Backend**: Rust (src-tauri/)

---

## 2. Comandos de Build y Desarrollo

### Comandos principales (frontend)

```bash
# Desarrollo
pnpm dev                          # Inicia el servidor Vite en http://localhost:1420
pnpm build                        # TypeScript check + build de producción
pnpm preview                      # Previsualizar build de producción
```

### Comandos Tauri

```bash
# Desarrollo Tauri (inicia frontend + backend)
pnpm tauri dev

# Build de producción Tauri
pnpm tauri build

# Ejecutar un solo test (si existe)
pnpm test
# o
pnpm tauri test
```

### Comandos Rust (directos)

```bash
# Dentro de src-tauri/
cd src-tauri && cargo check       # Verificar código sin compilar
cd src-tauri && cargo build        # Compilar
cd src-tauri && cargo test         # Ejecutar tests
cd src-tauri && cargo clippy       # Linter de Rust
```

### Verificar tipos TypeScript

```bash
pnpm build                         # Incluye vue-tsc --noEmit
```

---

## 3. Convenciones de Código - TypeScript/Vue

### Estructura de archivos

- Componentes Vue: `PascalCase.vue` o `kebab-case.vue`
- Archivos TypeScript: `camelCase.ts`
- Tipos/Interfaces: `camelCase.types.ts` o en el mismo archivo del módulo

### Imports

```typescript
// Usar comillas dobles
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { User } from "./types";

// Importaciones de relativa
import App from "./App.vue";
import { helper } from "../utils/helper";
```

### Naming Conventions

```typescript
// Variables y funciones: camelCase
const userName = "Juan";
function getUserById(id: string): User {}

// Constantes: UPPER_SNAKE_CASE (para valores de configuración)
const MAX_RETRIES = 3;

// Types/Interfaces/Enums: PascalCase
interface UserProfile { ... }
type ApiResponse<T> = { ... };
enum Status { ... }

// Componentes Vue: PascalCase en el template
<UserCard />, <SettingsDialog />

// Props de componentes: camelCase (JS) o kebab-case (HTML template)
defineProps<{ userName: string; userAge: number }>();
// Template: <UserCard user-name="Ana" />
```

### TypeScript Strict Mode

El proyecto tiene `strict: true` en tsconfig.json. Reglas activas:

- `noUnusedLocals: true` - No declarar variables sin usar
- `noUnusedParameters: true` - No tener parámetros sin usar
- `noFallthroughCasesInSwitch: true` - Todos los casos switch deben break/return

```typescript
// ❌ Incorrecto
const unused = "test";
function foo(x: number) { }

// ✅ Correcto
const userName = "Juan";
function getUser(id: string): User { return {} as User; }
```

### Componentes Vue 3

```vue
<script setup lang="ts">
// Imports al inicio
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

// Definir props con defineProps
const props = defineProps<{
  title: string;
  count?: number;
}>();

// Estado reactivo
const isLoading = ref(false);

// Computed
const doubledCount = computed(() => (props.count ?? 0) * 2);

// Funciones
async function fetchData() {
  isLoading.value = true;
  try {
    const result = await invoke<string>("command_name", { id: 1 });
  } catch (error) {
    console.error(error);
  } finally {
    isLoading.value = false;
  }
}
</script>

<template>
  <!-- Template aquí -->
</template>

<style scoped>
/* Estilos scoped por defecto */
</style>
```

### Manejo de Errores

```typescript
// Siempre usar try/catch con async/await
async function loadData() {
  try {
    const data = await fetchFromApi();
    return data;
  } catch (error) {
    console.error("Error al cargar datos:", error);
    throw error; // o manejar según el contexto
  }
}

// Para Promises, encadenar .catch()
fetchData().catch((error) => {
  console.error(error);
});
```

---

## 4. Convenciones de Código - Rust

### Estructura de archivos

```
src-tauri/
├── src/
│   ├── lib.rs              # Lógica principal (commands, plugins)
│   └── main.rs             # Entry point
├── Cargo.toml
└── tauri.conf.json
```

### Estilo de código

- **Indentación**: 4 espacios (no tabs)
- **Llaves**: Same-line para funciones, newline para structs/enums

```rust
// Función
fn greet(name: &str) -> String {
    format!("Hola, {}!", name)
}

// Struct
struct User {
    name: String,
    age: u32,
}

// Enum
enum Status {
    Active,
    Inactive,
}
```

### Imports

```rust
use serde::{Deserialize, Serialize};
use tauri::command;
```

### Naming Conventions

- Funciones/variables: `snake_case`
- Structs/Enums/Traits: `PascalCase`
- Constantes: `SCREAMING_SNAKE_CASE`
- Módulos: `snake_case`

### Macros y Atributos

```rust
#[tauri::command]
fn my_command(arg: String) -> Result<String, String> {
    Ok(arg)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![my_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## 5. Integración Tauri (Frontend <-> Backend)

### Llamar comandos Rust desde Vue

```typescript
import { invoke } from "@tauri-apps/api/core";

// En lib.rs de Rust
#[tauri::command]
fn my_command(id: u32, name: String) -> Result<MyResponse, String> {
    // lógica
}

// En Vue/TypeScript
const result = await invoke<MyResponse>("my_command", {
  id: 42,
  name: "ejemplo"
});
```

### Tipos compartidos

- Definir tipos TypeScript que correspondan a las structs de Rust
- Usar `serde` derive macros en Rust: `#[derive(Serialize, Deserialize)]`

---

## 6. Configuración Importante

### tsconfig.json (TypeScript)

```json
{
  "compilerOptions": {
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  }
}
```

### tauri.conf.json

- Configuración de la ventana, CSP, y bundle settings
- `devUrl`: http://localhost:1420
- `frontendDist`: ../dist

---

## 7. Errores Comunes a Evitar

1. **No deixar variables sin usar** - TypeScript lo marca como error
2. **No usar `any`** - Usar tipos específicos o `unknown`
3. **No olvidar el `.value`** al acceder a refs de Vue
4. **No usar `console.log`** en producción - usar logging apropiado
5. **No hardcodear secrets** - usar variables de entorno
6. **En Rust, siempre hacer `.expect()` o manejar el `Result`**

---

## 8. IDE Recomendado

- **VS Code** con extensiones:
  - Vue - Official (Volar)
  - Tauri
  - rust-analyzer
  - ESLint
  - Prettier
