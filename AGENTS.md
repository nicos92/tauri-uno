# AGENTS.md - Guía para Agentes de Código

Este documento proporciona instrucciones y convenciones para agentes de código que operan en este repositorio.

---

## 1. Resumen del Proyecto

- **Stack**: Tauri 2 + Vue 3 + TypeScript + Vite + Pinia + Vue Router
- **Package Manager**: pnpm
- **Frontend**: TypeScript strict mode habilitado
- **Backend**: Rust con arquitectura limpia (Clean Architecture)
- **Base de datos**: SQLite con rusqlite
- **Frontend**: Vue 3 con arquitectura limpia (Domain-Driven Design simplificado)

---

## 2. Arquitectura del Proyecto

### Backend Rust (Clean Architecture)

```
src-tauri/src/
├── domain/           # Entidades y traits de repositorio
│   ├── entities/     # User, Permission
│   └── repositories/# UserRepository trait
├── application/     # Casos de uso/Servicios
│   └── services/    # UserService
├── infrastructure/  # Implementaciones concretas
│   ├── database/    # Conexión SQLite
│   ├── repositories/# SqliteUserRepository
│   └── error.rs     # AppError enum
└── api/             # Commands de Tauri
    └── commands/    # invoke handlers
```

### Frontend Vue (Clean Architecture)

```
src/
├── domain/           # Tipos e interfaces
│   ├── entities/    # User, Permission types
│   └── interfaces/  # IUserRepository
├── application/     # Casos de uso
│   └── usecases/   # Login, CreateUser, etc.
├── infrastructure/ # Implementaciones
│   └── api/        # UserApiRepository
└── presentation/   # Capa UI
    ├── layouts/    # MainLayout con sidebar
    ├── pages/      # Login, Home, Users, Permissions, Settings
    ├── stores/     # Pinia stores (auth, users, permissions)
    └── router/     # Vue Router config
```

---

## 3. Comandos de Build y Desarrollo

### Comandos principales (frontend)

```bash
pnpm dev                          # Inicia el servidor Vite en http://localhost:1420
pnpm build                        # TypeScript check + build de producción
pnpm preview                      # Previsualizar build de producción
```

### Comandos Tauri

```bash
pnpm tauri dev                    # Desarrollo Tauri (frontend + backend)
pnpm tauri build                  # Build de producción Tauri
```

### Comandos Rust (directos)

```bash
cd src-tauri && cargo check       # Verificar código sin compilar
cd src-tauri && cargo build        # Compilar
cd src-tauri && cargo test         # Ejecutar tests
cd src-tauri && cargo clippy       # Linter de Rust
```

---

## 4. Convenciones de Código - TypeScript/Vue

### Estructura de archivos

- Componentes Vue: `PascalCase.vue`
- Archivos TypeScript: `camelCase.ts`
- Tipos/Interfaces: `camelCase.types.ts` o en el mismo archivo del módulo

### Imports

```typescript
// Usar comillas dobles
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { User } from "../../domain/entities";

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
```

### TypeScript Strict Mode

El proyecto tiene `strict: true` en tsconfig.json. Reglas activas:

- `noUnusedLocals: true` - No declarar variables sin usar
- `noUnusedParameters: true` - No tener parámetros sin usar
- `noFallthroughCasesInSwitch: true` - Todos los casos switch deben break/return

### Componentes Vue 3

```vue
<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

const props = defineProps<{
  title: string;
  count?: number;
}>();

const isLoading = ref(false);

const doubledCount = computed(() => (props.count ?? 0) * 2);

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

---

## 5. Convenciones de Código - Rust

### Estructura de archivos

```
src-tauri/src/
├── domain/entities/    # Structs con derive Serialize
├── domain/repositories/# Traits
├── application/services# Lógica de negocio
├── infrastructure/    # Implementaciones concretas
└── api/commands/       # Tauri commands
```

### Estilo de código

- **Indentación**: 4 espacios (no tabs)
- **Llaves**: Same-line para funciones, newline para structs/enums

```rust
fn greet(name: &str) -> String {
    format!("Hola, {}!", name)
}

struct User {
    name: String,
    age: u32,
}

enum Status {
    Active,
    Inactive,
}
```

### Naming Conventions

- Funciones/variables: `snake_case`
- Structs/Enums/Traits: `PascalCase`
- Constantes: `SCREAMING_SNAKE_CASE`

### Macros y Atributos

```rust
#[tauri::command]
fn my_command(arg: String) -> Result<String, AppError> {
    Ok(arg)
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}
```

---

## 6. Integración Tauri (Frontend <-> Backend)

### Llamar comandos Rust desde Vue

```typescript
import { invoke } from "@tauri-apps/api/core";

const result = await invoke<UserResponse>("create_user", {
  request: { username: "test", password: "123" }
});
```

### Tipos compartidos

- Definir tipos TypeScript que correspondan a las structs de Rust
- Usar `serde` derive macros en Rust: `#[derive(Serialize, Deserialize)]`

---

## 7. Modelos de Base de Datos

### Usuarios
- `id`: INTEGER PRIMARY KEY
- `username`: TEXT UNIQUE
- `password`: TEXT (hashed con bcrypt)
- `active`: INTEGER (0/1)
- `created_at`: TEXT (ISO 8601)
- `modified_at`: TEXT (ISO 8601)

### Permisos
- `id`: INTEGER PRIMARY KEY
- `permission`: TEXT UNIQUE
- `created`: TEXT (ISO 8601)

### user_permissions (relación muchos a muchos)
- `user_id`: INTEGER FK
- `permission_id`: INTEGER FK

---

## 8. API Commands (Tauri)

| Command | Descripción |
|---------|-------------|
| `login` | Autenticar usuario |
| `create_user` | Crear nuevo usuario |
| `get_all_users` | Listar todos los usuarios |
| `update_user` | Actualizar usuario |
| `delete_user` | Eliminar usuario |
| `get_user_permissions` | Obtener permisos de un usuario |
| `get_all_permissions` | Listar todos los permisos |
| `add_permission_to_user` | Asignar permiso a usuario |
| `remove_permission_from_user` | Quitar permiso a usuario |
| `create_permission` | Crear nuevo permiso |

---

## 9. Errores Comunes a Evitar

1. **No dejar variables sin usar** - TypeScript lo marca como error
2. **No usar `any`** - Usar tipos específicos o `unknown`
3. **No olvidar el `.value`** al acceder a refs de Vue
4. **En Rust, siempre manejar `Result` con `?` o match**
5. **No hardcodear secrets** - usar variables de entorno

---

## 10. IDE Recomendado

- **VS Code** con extensiones:
  - Vue - Official (Volar)
  - Tauri
  - rust-analyzer
  - ESLint
  - Prettier
