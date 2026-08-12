# Cotización del Dólar (Oficial y Blue) con polling configurable

## Problem

La aplicación necesita un módulo que obtenga las cotizaciones actualizadas del
Dólar Oficial y del Dólar Blue en Argentina desde una API pública
(`https://dolarapi.com/v1/dolares`) y las muestre en la interfaz.

Los requisitos funcionales son:

1. **Consulta bajo demanda**: un comando manual que consulta la API, persiste
   los valores en la base de datos local y devuelve la cotización actualizada.
2. **Lectura de la última cotización persistida**: un comando que retorna lo
   que hay almacenado (aunque la API esté caída, se muestra la última lectura).
3. **Actualización automática**: un *polling* en segundo plano ejecutado por el
   backend con un intervalo configurable, que también puede **pausarse**.
4. **Sin duplicación de polling**: si el usuario cambia el intervalo
   dinámicamente, no debe quedar más de una tarea de polling corriendo.
5. **Persistencia tipo "última lectura"**: por cada tipo de dólar
   (`oficial`, `blue`) se conserva una sola fila; cada lectura exitosa la
   reemplaza (upsert).

## Input

- `fetch_dollar_rates_manual(user_id)` — sin parámetros de dominio; dispara una
  lectura HTTP + persistencia.
- `get_latest_dollar_rates(user_id)` — sin parámetros de dominio; retorna lo
  persistido.
- `update_polling_interval(user_id, seconds: u64)` — `0` pausa el polling;
  cualquier otro valor cambia el intervalo (en segundos).
- La API pública responde un arreglo JSON donde cada elemento tiene
  `casa`, `compra`, `venta` y `fecha_actualizacion`.

## Output

- `fetch_dollar_rates_manual` → `Vec<DollarRate>` recién obtenido y persistido.
- `get_latest_dollar_rates` → `Vec<DollarRate>` almacenado en DB (posiblemente
  vacío en la primera ejecución).
- `update_polling_interval` → `u64` con el intervalo aplicado.
- Evento `dollar-rates-updated` (payload `Vec<DollarRate>`) emitido por el
  polling en segundo plano para que el frontend se actualice sin consultar.
- Evento `dollar-rates-fetch-error` (payload `String`) cuando una lectura
  automática falla.
- Errores: `AppError::DollarFetchError` si la red o la API fallan;
  `AppError::PermissionDenied` si el usuario no tiene `ver_dolar`.

## Constraints

- `DollarRate { dollar_type, buy_price, sell_price, updated_at }`: el
  `dollar_type` solo puede ser `oficial` o `blue` (la API devuelve más casas:
  bolsa, contado con liqui, tarjeta, cripto, mayorista; se filtran).
- `user_id` es obligatorio en cada comando y cada uno exige el permiso
  `ver_dolar` (patrón del proyecto, ver AGENTS.md §9.8).
- La base es SQLite con una conexión global `Mutex<Connection>` (no `Sync`):
  las escrituras desde el hilo de polling deben ir por
  `spawn_blocking` para no bloquear el runtime asíncrono.
- Las fechas viajan como strings RFC3339/ISO 8601 (`fecha_actualizacion` se
  pasa tal cual) para evitar problemas de serialización Rust ↔ JS.

## Proposed Solution

Un servicio `DollarService` con dos responsabilidades:

1. **Cliente HTTP** (`DollarHttpClient`): `GET https://dolarapi.com/v1/dolares`,
   con timeout, y mapeo del payload a `Vec<DollarRate>` vía la función pura
   `parse_api_response`.
2. **Persistencia**: repositorio `DollarRateRepository` con `upsert`
   (INSERT ... ON CONFLICT DO UPDATE) para mantener solo la última lectura.

El **polling** se implementa como **una única tarea de larga vida** manejada en
`DollarAppState` (estado Tauri). En lugar de abortar y re-crear hilos al
cambiar el intervalo, la tarea escucha un canal `tokio::sync::watch`:

- El estado guarda `interval_seconds`, un `watch::Sender<u64>` y un
  `JoinHandle`.
- Cada cambio de intervalo hace `sender.send(seconds)`; esto **despierta** la
  tarea dormida y esta relee el nuevo intervalo en la siguiente iteración.
- `seconds == 0` = pausado: la tarea no consulta y se queda esperando en
  `rx.changed()` hasta que se reanude.
- Solo existe un `JoinHandle`: nunca se duplica la tarea (respuesta al
  requerimiento "no duplicar hilos de polling").

Tras cada lectura exitosa la tarea emite `dollar-rates-updated` con los valores
persistidos; si falla, emite `dollar-rates-fetch-error` y conserva la última
lectura en DB (el frontend puede seguir mostrando valores viejos).

### Alternatives Considered

1. **Abort + respawn del `JoinHandle` en cada cambio de intervalo**: simple de
   entender, pero al cancelar una tarea que está en mitad de un fetch puede
   haber una ventana de solapamiento con la nueva tarea y pierde el historial
   de la variable de configuración. La tarea única con `watch` no cancela
   lecturas en vuelo y cambia de intervalo sin churn.
2. **`tokio::interval` con lectura previa del estado compartido en cada tick**
   (ej. `Arc<AtomicU64>` para el intervalo): similar a la solución elegida,
   pero un `AtomicU64` no puede "despertar" la tarea de inmediato; un cambio
   de intervalo solo se aplica al completar el tick actual. Con `watch` el
   despertar es inmediato.
3. **Polling en el frontend** (`setInterval` que llame a
   `get_latest_dollar_rates`): duplica la lógica y la ventana de refresco
   depende de que haya un frontend vivo; el backend es la fuente de verdad.

| Approach | Tarea única | Despertar inmediato | Duplicados | Complejidad |
|----------|-------------|---------------------|------------|-------------|
| `watch` + sleep con `select` | Sí | Sí | Imposible | Media |
| Abort + respawn | No | Sí | Ventana breve | Media |
| `AtomicU64` + `tokio::interval` | Sí | No (siguiente tick) | Imposible | Baja |
| Polling en frontend | — | — | Fácil | Baja |

### Why This Approach

- Respeta clean architecture del proyecto: la capa de estado/comandos solo
  orquesta; el fetching y el upsert viven en `application/services` y
  `infrastructure/repositories`.
- La tarea única + `watch` satisface de forma directa "actualizar o pausar el
  temporizador" y "no duplicar hilos de polling".
- El upsert (`ON CONFLICT(dollar_type) DO UPDATE`) es idempotente y mantiene
  exactamente una fila por tipo.

## Algorithm

```
estado:
  interval_seconds : u64      (0 = pausado)
  sender          : watch::Sender<u64>
  handle          : JoinHandle<()>   (una sola tarea)

start_polling(segundos):
    si handle es None:
        crear canal watch(segundos)
        spawn( polling_loop(app, service, receiver) )
    interval_seconds = segundos
    sender.send(segundos)            # despierta la tarea si está dormida

polling_loop(app, service, receiver):
    repetir:
        interval = receiver.ultimo_valor()

        si interval == 0:
            esperar receiver.changed()      # pausa hasta reanudar
            continuar

        resultado = service.fetch_and_persist()
        si ok(rates):
            app.emit("dollar-rates-updated", rates)
        si err:
            app.emit("dollar-rates-fetch-error", "mensaje")

        # duerme `interval` o se despierta antes si cambia la config
        select:
            caso sleep(interval)
            caso receiver.changed()

upsert(rate):
    INSERT INTO dollar_rates (dollar_type, buy_price, sell_price, updated_at)
    VALUES (rate...)
    ON CONFLICT(dollar_type) DO UPDATE SET
        buy_price = excluded.buy_price,
        sell_price = excluded.sell_price,
        updated_at = excluded.updated_at

fetch_and_persist():
    rates = GET https://dolarapi.com/v1/dolares   # con timeout
    rates = filtrar(casa ∈ {oficial, blue})
    rates = mapear(compra→buy_price, venta→sell_price,
                   fecha_actualizacion→updated_at)
    spawn_blocking:
        por cada rate: upsert(rate)
    retornar rates
```

### Pseudocode

```text
function startPolling(seconds):
    if handle is None:
        (sender, receiver) = watchChannel(seconds)
        handle = spawn(pollingLoop(receiver))

    intervalSeconds = seconds
    sender.send(seconds)

function pollingLoop(receiver):
    loop:
        interval = receiver.current()

        if interval == 0:
            await receiver.changed()       # paused
            continue

        result = await fetchAndPersist()

        if result is success(rates):
            emitEvent("dollar-rates-updated", rates)
        else:
            emitEvent("dollar-rates-fetch-error", message)

        select:
            wait(sleep(interval))          # timer expires
            or receiver.changed()          # config changed → restart loop

function fetchAndPersist():
    body = GET "https://dolarapi.com/v1/dolares"   # timeout 10 s
    rates = []
    for item in parseJson(body):
        if item.casa in {"oficial", "blue"}:
            rates.push({ dollar_type: item.casa,
                         buy_price: item.compra,
                         sell_price: item.venta,
                         updated_at: item.fecha_actualizacion })
    sort(rates)                                  # "blue" < "oficial"
    for rate in rates:                           # in spawn_blocking
        upsert(rate)
    return rates

function upsert(rate):
    INSERT INTO dollar_rates (dollar_type, buy_price, sell_price, updated_at)
    VALUES (rate.dollar_type, rate.buy_price, rate.sell_price, rate.updated_at)
    ON CONFLICT(dollar_type) DO UPDATE SET
        buy_price = excluded.buy_price,
        sell_price = excluded.sell_price,
        updated_at = excluded.updated_at
```

## Complexity

### Time

- Por tick de polling: O(k), donde `k` es el número de tipos de dólar que
  devuelve la API (filtrado: k ≈ 2). El upsert usa la PK
  `dollar_type` (índice de tabla): O(1) por fila. El parseo es O(n) sobre el
  arreglo de la API, con n pequeño y acotado.
- `get_latest_dollar_rates`: O(k) = O(1) con la PK.
- El costo de red domina en la práctica (una petición cada `interval`
  segundos).

### Space

- O(k) para las cotizaciones en memoria (k ≈ 2) y O(k) en DB (una fila por
  tipo). La tarea de polling usa memoria constante además de los valores en
  vuelo durante una lectura.

## Edge Cases

- `seconds == 0` → polling pausado: no se consulta la API y no se emiten
  eventos hasta reanudar.
- Cambio de intervalo mientras la tarea duerme → `watch` la despierta y aplica
  el nuevo valor en la misma iteración; nunca hay dos tareas.
- La API está caída / sin red → `fetch_and_persist` devuelve
  `DollarFetchError`; el polling emite `dollar-rates-fetch-error` y **conserva**
  la última lectura en DB (el frontend muestra los valores viejos con su
  `updated_at` original).
- Primera ejecución sin datos → `get_latest_dollar_rates` retorna `[]`; la UI
  muestra "Sin datos".
- La API devuelve casas adicionales (bolsa, cripto, etc.) → se filtran; solo
  se persisten `oficial` y `blue`.
- La API devuelve un payload inválido → error de parseo mapeado a
  `DollarFetchError`; no se toca la DB.
- `fecha_actualizacion` en formato no estándar → se persiste y muestra tal
  cual como string (sin parsear en backend).

## Examples

### Example 1

Input:

```text
fetch_dollar_rates_manual(1)   # admin
# GET https://dolarapi.com/v1/dolares
# [ { casa: "oficial", compra: 1000, venta: 1040, ... },
#   { casa: "blue",    compra: 1200, venta: 1240, ... },
#   { casa: "bolsa",   compra: 1100, venta: 1140, ... } ]
```

Output:

```text
Ok([
  DollarRate { dollar_type: "blue",    buy_price: 1200, sell_price: 1240, ... },
  DollarRate { dollar_type: "oficial", buy_price: 1000, sell_price: 1040, ... }
])
```

Explanation:

Solo `oficial` y `blue` se filtran y persisten (la casa "bolsa" se descarta).
Ambas filas quedan en `dollar_rates` vía upsert.

### Example 2

Input:

```text
get_latest_dollar_rates(1)
```

Output (API caída, última lectura previa):

```text
Ok([
  DollarRate { dollar_type: "blue",    buy_price: 1200, sell_price: 1240, updated_at: "2025-01-15T12:30:00.000-03:00" },
  DollarRate { dollar_type: "oficial", buy_price: 1000, sell_price: 1040, updated_at: "2025-01-15T12:30:00.000-03:00" }
])
```

Explanation:

Los valores devueltos provienen de la última persistencia; el `updated_at`
sigue siendo la fecha de la última lectura exitosa.

### Example 3

Input:

```text
update_polling_interval(1, 60)   # antes: 300 s
```

Output:

```text
Ok(60)
```

Explanation:

El `sender` envía 60 y despierta la tarea dormida; el siguiente ciclo usa 60 s.
No se creó una segunda tarea: el `handle` ya existía y solo se reutilizó.

## Implementation

Implementado en:

- `src-tauri/src/domain/entities/dollar_rate.rs` — entidad `DollarRate`.
- `src-tauri/src/domain/repositories/dollar_rate_repository.rs` — trait
  `DollarRateRepository`.
- `src-tauri/src/infrastructure/repositories/dollar_rate_repository.rs` —
  implementación SQLite (upsert).
- `src-tauri/src/application/services/dollar_service.rs` — cliente HTTP,
  `parse_api_response`, `fetch_and_persist`, `get_latest`.
- `src-tauri/src/api/commands/dollar_commands.rs` — `DollarAppState`, tarea de
  polling (`watch` + `select`), comandos Tauri.
- `src-tauri/src/infrastructure/database/mod.rs` — tabla `dollar_rates`.
- `src-tauri/src/infrastructure/error.rs` — variante `DollarFetchError`.
- `src-tauri/src/domain/entities/permission_code.rs` + seeds — permiso
  `ver_dolar`.
- `src-tauri/src/domain/entities/audit_log.rs` — `AuditScreen::Dolar`.
- `src-tauri/src/lib.rs` — estado, comando y arranque del polling (`setup`).
- Frontend: `src/domain/entities/types.ts`, `permissions.ts`,
  `src/presentation/composables/useDollarRates.ts`, `usePermissions.ts`,
  `src/presentation/pages/DolarPage.vue`, `router/index.ts`,
  `layouts/MainLayout.vue`, `pages/AuditoriaPage.vue`, `public/svg/dolar.svg`.

## Validation

- [ ] Cargo test: parseo de respuesta de la API
- [ ] Cargo test: repositorio (upsert idempotente, `find_all`, `find_by_type`)
- [ ] Cargo test: propagación de `DollarFetchError`
- [ ] `cargo check` / `cargo test` / `cargo clippy --lib --tests` sin errores
- [ ] `pnpm build` (vue-tsc + vite) sin errores
- [ ] Verificación manual del polling (intervalo, pausa, manual)

## Observations

- El frontend refleja las lecturas automáticas mediante el evento
  `dollar-rates-updated` (el backend empuja); no hay un segundo temporizador en
  el frontend que duplique el polling.
- La fecha se serializa como string ISO 8601 tanto en Rust como en JS: no hay
  conversión de tipos de fecha a través del puente `invoke`.
- Para no bloquear el runtime asíncrono, la escritura en DB dentro del polling
  usa `tauri::async_runtime::spawn_blocking` (rusqlite no es `Send`).
