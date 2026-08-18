# Análisis Técnico: Deshacer Cambios de Precios de Costo

## 1. Modelo de datos existente
Tabla stock (única tabla que almacena el precio de costo)
CREATE TABLE IF NOT EXISTS stock (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    id_articulo INTEGER NOT NULL,
    cantidad REAL NOT NULL,
    costo REAL NOT NULL,
    ganancia REAL NOT NULL,
    FOREIGN KEY (id_articulo) REFERENCES articulos(id)
);
- El costo es un REAL NOT NULL sin CHECK constraints, sin triggers, sin índices más allá del PK.
- No existe UNIQUE constraint en id_articulo — la unicidad se garantiza en código (StockService::create verifica find_by_articulo antes de insertar).
- No existen triggers en ninguna tabla de la base de datos.
- No existe ningún historial de precios ni tabla de operaciones. El costo se sobreescribe silenciosamente.
Cadena de relaciones para filtros
stock → articulos (via id_articulo)
  articulos → sub_categorias (via id_sub_categoria)
    sub_categorias → categorias (via id_categoria)
  articulos → proveedores (via id_proveedor)
Tabla audit_logs existente
CREATE TABLE IF NOT EXISTS audit_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    username TEXT NOT NULL,
    screen TEXT NOT NULL,
    action TEXT NOT NULL,
    detail TEXT,
    created_at TEXT NOT NULL
);
- Solo almacena strings descriptivos (detail). No registra valores anteriores/nuevos por fila.
- Se auto-purga a los 90 días.
- Es la única tabla de auditoría del sistema.

## 2. Flujo actual de actualización de precios
Flujo completo: presionar "Aplicar Cambios" hasta persistir
1. Usuario presiona "Aplicar Cambios" en ActualizarCostoPage.vue
       ↓
2. useConfirm() solicita confirmación ("danger" variant)
       ↓
3. stockStore.applyCostoPercentage({ porcentaje, id_categoria, ... })
       ↓
4. StockUseCase.applyCostoPercentage(request) → pasa al repo
       ↓
5. StockApiRepository.applyCostoPercentage(request)
   → invoke("apply_costo_percentage_stock", { userId, request })
       ↓
6. [Rust] apply_costo_percentage_stock command:
   a. check_permission(user_id, UpdateStock)
   b. StockService::apply_costo_percentage(porcentaje, filtros)
      - Valida: no NaN, no 0, no < -100, no Infinity
      - Llama repository.apply_costo_percentage(porcentaje, filtros)
   c. SqliteStockRepository: ejecuta un único UPDATE SQL:
      UPDATE stock SET costo = ROUND(costo * (1.0 + porcentaje / 100.0), 2)
      WHERE id IN (subquery con JOINs y filtros)
      → Retorna count de filas afectadas
   d. Si count == 0 → AppError::BulkUpdateNoMatches
   e. log_audit(user_id, Stock, Update, "Actualización masiva: +20% (125 artículos)")
   f. Retorna { updated_count: 125 }
       ↓
7. Frontend muestra toast de éxito y refresca stock
Qué se pierde
- Los valores anteriores de cada costo se sobreescriben sin registro.
- El audit_logs solo almacena: "Actualización masiva: +20% (125 artículos)" — no los 125 valores anteriores.
- No hay forma de revertir la operación una vez ejecutada.

## 3. Alternativa A — Revertir mediante cálculo matemático

Análisis
La idea es: si se aplicó +20%, se revierte con $1.200 / 1.20 = $1.000.
Ventajas
- No requiere tablas nuevas.
- No requiere almacenamiento adicional.
- Implementación mínima.
Desventajas
- Redondeos: ROUND(1000 * 1.20, 2) = 1200.00. Pero ROUND(1200.00 / 1.20, 2) puede dar 999.99 o 1000.01 dependiendo de la representación en punto flotante de SQLite. Con 100 artículos, los errores de redondeo se acumulan.
- Múltiples operaciones: si se aplicó +20% y luego +10%, el costo final es $1000 * 1.20 * 1.10 = $1.320. Para revertir la última (+10%), sería $1.320 / 1.10 = $1.200. Pero esto requiere recordar el porcentaje exacto de cada operación en orden inverso.
- Modificación manual posterior: si un usuario cambió manualmente $1.200 → $1.350 entre la operación y el undo, el cálculo inverso restauraría un valor incorrecto (o intentaría).
- Diferentes artículos, mismo porcentaje: el cálculo inverso funciona individualmente, pero si un artículo fue modificado manualmente entre operaciones, el cálculo se invalida para ese artículo.
- Porcentajes combinados: si se aplicaron +20% y luego +5% en la misma categoría, revertir el +5% con cálculo inverso es posible, pero requiere conocer exactamente qué porcentaje se aplicó en cada paso.
- Operaciones no porcentuales: la funcionalidad futura podría incluir "asignar costo fijo", "igualar a otro artículo", etc. El cálculo inverso no funcionaría.
Conclusión
Descartada como estrategia principal. El cálculo matemático inverso no puede garantizar la exactitud del valor anterior real. Es frágil y acumula errores. No cumple con la prioridad #1 (integridad de datos) ni #2 (correcto comportamiento de Undo).
4. Alternativa B — Guardar el valor anterior (per-row snapshot)
Análisis
Registrar costo_anterior y costo_nuevo de cada artículo al momento de la operación.
Estructura conceptual
stock_cost_history
    id
    id_stock
    costo_anterior
    costo_nuevo
    operacion_id  (FK a una tabla de operaciones)
    timestamp
Ventajas
- El valor anterior se almacena tal cual, sin cálculos.
- Permite restore exacto.
Desventajas
- Sin agrupación lógica: si se modifican 125 artículos, se generan 125 registros independientes. No hay forma de saber que pertenecen a la misma operación.
- Deshacer parcial: sin una cabecera de operación, no se puede deshacer "la última operación completa" de forma atómica — habría que buscar todos los registros con el mismo timestamp, lo cual es impreciso.
- No escala a historial: sin cabecera de operación, no se puede consultar "historial de operaciones" ni saber "qué hizo el usuario a las 15:00".
Conclusión
Insuficiente por sí sola. Necesita complementarse con una tabla de cabecera (operación). Ver Alternativa C.
### 5. Alternativa C — Historial de operaciones (recomendada)
Análisis
Una arquitectura basada en cabecera + detalle:
cost_update_operations (cabecera)
    ├── id
    ├── user_id
    ├── porcentaje
    ├── filtro_categoria (nullable)
    ├── filtro_sub_categoria (nullable)
    ├── filtro_proveedor (nullable)
    ├── affected_count
    ├── estado ('aplicada' | 'deshecha')
    ├── created_at
    └── undone_at (nullable)

cost_update_items (detalle)
    ├── id
    ├── operation_id (FK → cost_update_operations)
    ├── id_stock
    ├── costo_anterior
    ├── costo_nuevo
Ventajas
- Restore exacto: el costo_anterior de cada item es el valor real que tenía el stock antes de la operación.
- Operación atómica lógica: los 125 items pertenecen a una sola operation_id.
- Deshacer seguro: UPDATE stock SET costo = item.costo_anterior WHERE id = item.id_stock, dentro de una transacción que también cambia estado a 'deshecha'.
- Auditoría completa: se puede consultar qué operaciones se hicieron, cuándo, por quién, y qué artículos afectaron.
- Extensible: permite futuras funcionalidades (ver historial, deshacer operación específica, rehacer).
- Filtros almacenados: la cabecera guarda los filtros originales, útil para auditoría y debugging.
- No depende de cálculos: el restore usa el valor real, no un cálculo inverso.
Desventajas
- Tablas nuevas (2).
- Incremento de tamaño en DB (proporcional al número de operaciones bulk × artículos afectados).
- UPDATE de stock requiere una transacción más compleja (actualizar stock + insertar items + crear cabecera).
Conclusión
Es la solución recomendada. Cumple con todas las prioridades: integridad, correctitud, atomicidad, trazabilidad, Clean Architecture, extensibilidad.
6. Qué significa "último cambio"
Definición precisa
"Último cambio" = la operación más reciente con estado 'aplicada' en cost_update_operations, ordenada por created_at DESC, id DESC.
Stack de operaciones
Operación 1 → +10%  (estado: aplicada)
Operación 2 → +20%  (estado: aplicada)
Operación 3 → -5%   (estado: aplicada)
                  ↑
                Undo → deshace solo la Operación 3
Después del primer Undo:
Operación 1 → +10%  (estado: aplicada)
Operación 2 → +20%  (estado: aplicada)
Operación 3 → -5%   (estado: deshecha)
Un segundo Undo deshace la Operación 2.
Consideraciones
- Solo se pueden deshacer operaciones en orden FIFO inverso (LIFO): la más reciente primero.
- No se permite "deshacer operación 1 saltándose la 2" — esto introduciría inconsistencias si la operación 2 dependió de los valores que la operación 1 dejó.
- El estado 'deshecha' es terminal: una operación deshecha no se puede "re-deshacer" (pero sí se puede "rehacer" aplicando el porcentaje original en una nueva operación, futuramente).
7. Modificaciones posteriores
Escenario
Precio inicial: $1.000
Operación 1: +20% → $1.200
Modificación manual: $1.200 → $1.350
Undo Operación 1: ¿qué debería ocurrir?
Alternativa evaluada: Restaurar $1.000
Descartada. Si el usuario modificó manualmente el precio a $1.350, restaurar $1.000 borraria la modificación manual sin aviso. El usuario perdería trabajo no relacionado con la operación de bulk update.
Alternativa evaluada: No permitir Undo
Demasiado restrictiva. Podría haber casos legítimos donde el usuario quiere deshacer el bulk update y la modificación manual fue sobre un artículo diferente.
Alternativa recomendada: Advertir y bloquear por artículo
Al momento de deshacer, el sistema verifica si el costo_actual de cada artículo coincide con el costo_nuevo registrado en la operación:
SELECT item.id, item.costo_nuevo, stock.costo
FROM cost_update_items item
JOIN stock ON item.id_stock = stock.id
WHERE item.operation_id = ?1
  AND stock.costo != item.costo_nuevo
Si algún artículo fue modificado posteriormente:
- Opción A: Bloquear el undo completo y mostrar: "No se puede deshacer: 3 artículos fueron modificados manualmente después de la operación."
- Opción B (recomendada): Ofrecer undo parcial — deshacer solo los artículos cuyo costo coincide con el esperado, y reportar cuáles se omitieron.
Recomendación: Opción A (bloqueo total) por seguridad. La Opción B es más flexible pero introduce complejidad significativa y riesgo de confusión. Se puede implementar como mejora futura.
Nuevo AppError necesario
CostUpdateModifiedAfter {
    modified_count: i64,
}
// user_message: "No se puede deshacer: {modified_count} artículo(s) fueron modificados después de la operación."
8. Operaciones parciales
Escenario
Operación afecta 100 artículos, error en el artículo #57.
Análisis
El apply_costo_percentage actual ejecuta un único UPDATE SQL:
UPDATE stock SET costo = ROUND(costo * (1.0 + ?1 / 100.0), 2) WHERE id IN (subquery)
SQLite ejecuta esto como una operación atómica a nivel de statement. No puede fallar a mitad de camino (no hay error parcial dentro de un solo UPDATE). Si hay un error (constraint violation, disco lleno), ninguna fila se actualiza.
Para la nueva implementación con historial
El flujo debe ser una única transacción:
BEGIN TRANSACTION;
  -- 1. Crear cabecera de operación
  INSERT INTO cost_update_operations (...) VALUES (...);
  -- 2. Insertar items (valores anteriores)
  INSERT INTO cost_update_items (...) 
    SELECT ?, s.id, s.costo, ROUND(s.costo * (1 + ?2 / 100), 2)
    FROM stock s WHERE s.id IN (subquery);
  -- 3. Actualizar stock
  UPDATE stock SET costo = ROUND(costo * (1.0 + ?1 / 100.0), 2)
    WHERE id IN (subquery);
  -- 4. Actualizar affected_count en cabecera
  UPDATE cost_update_operations SET affected_count = (SELECT COUNT(*) FROM cost_update_items WHERE operation_id = ?);
COMMIT;
Si cualquier paso falla, SQLite revierte todo. No hay estados parciales.
El usuario recibe要么 éxito completo要么 error completo.
9. Concurrencia y consistencia
Escenarios a considerar
Escenario	Solución
Dos bulk updates simultáneos	El Mutex<StockService> serializa las operaciones a nivel de Tauri state. Solo un comando de stock se ejecuta a la vez.
Bulk update + edición manual simultánea	El Mutex lo previene. La edición individual (update_stock) también pasa por el mismo mutex.
Undo + bulk update simultáneos	El mismo mutex lo previene.
App cierra durante la operación	La transacción SQL se revierte automáticamente por SQLite. No hay estado parcial.
App cierra después de actualizar pero antes de registrar historial	Imposible si la transacción incluye tanto la actualización como el registro del historial en el mismo BEGIN/COMMIT.
Estrategia transaccional
Toda la operación (crear cabecera + insertar items + actualizar stock + marcar estado) debe ocurrir dentro de una única transacción SQLite. El Mutex<StockService> ya serializa el acceso a nivel de aplicación, lo que previene race conditions a nivel de Tauri.
Estrategia de Undo
BEGIN TRANSACTION;
  -- 1. Verificar que la operación puede deshacerse
  -- 2. Verificar que ningún artículo fue modificado después
  -- 3. Restaurar costos
  UPDATE stock SET costo = (
      SELECT item.costo_anterior FROM cost_update_items item WHERE item.id_stock = stock.id AND item.operation_id = ?1
  ) WHERE id IN (SELECT item.id_stock FROM cost_update_items item WHERE item.operation_id = ?1);
  -- 4. Marcar operación como deshecha
  UPDATE cost_update_operations SET estado = 'deshecha', undone_at = datetime('now') WHERE id = ?1;
COMMIT;
10. Casos de uso necesarios
Siguiendo el patrón existente (casos de uso atómicos, cada uno con una responsabilidad clara):
Caso de uso	Responsabilidad
GetCostUpdatePreview	Obtener la vista previa de artículos afectados por un filtro + porcentaje
ApplyCostoPercentage	Aplicar la modificación porcentual, crear cabecera + items + actualizar stock, todo en una transacción
GetLastUndoableOperation	Obtener la operación más reciente con estado 'aplicada' (para mostrar en el botón de Undo)
UndoCostUpdateOperation	Deshacer la última operación: verificar que puede deshacerse, restaurar valores, marcar como deshecha
Nota: GetCostUpdatePreview y ApplyCostoPercentage ya existen parcialmente. Se extienden para incluir el historial.
No se crea un "gigante" PriceService. El StockService existente se mantiene para operaciones CRUD de stock. Los nuevos casos de uso se agregan como métodos adicionales al StockService o como un nuevo CostUpdateService dedicado (ver §11).
11. Clean Architecture y SOLID
Opción 1: Agregar al StockService existente
Pro: mantiene la cohesión de stock en un solo servicio.
Contra: StockService ya tiene 8 métodos. Agregar historial + undo lo hace más grande.
Opción 2: Nuevo CostUpdateService (recomendada)
Un servicio dedicado que maneja exclusivamente:
- Creación de operaciones de actualización de costo
- Deshacer operaciones
- Consulta de última operación
Se inyecta el StockRepository (para leer/actualizar stock) y un nuevo CostUpdateRepository (para leer/escribir historial).
CostUpdateService
    ├── apply_costo_percentage(porcentaje, filtros) → CostUpdateOperation
    ├── get_last_undoable() → Option<CostUpdateOperation>
    ├── undo_operation(operation_id) → ()
    └── get_preview(porcentaje, filtros) → Vec<StockPreview>
Capas respetadas
Vue/TS
  ↓ invoke
Tauri Command (adaptador)
  ↓
CostUpdateService (caso de uso / lógica de negocio)
  ↓
CostUpdateRepository (acceso a datos historial)
StockRepository (acceso a datos stock)
  ↓
SQLite
Dependency Inversion
CostUpdateService depende de traits (CostUpdateRepository, StockRepository), no de implementaciones concretas. La inyección se registra en lib.rs con .manage(...).
SRP
- StockService → CRUD de stock (create, update, delete, getPrecioVenta)
- CostUpdateService → operaciones bulk de costo + historial + undo
12. Modelo de historial propuesto
Tabla cost_update_operations (cabecera)
CREATE TABLE IF NOT EXISTS cost_update_operations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    porcentaje REAL NOT NULL,
    filtro_categoria INTEGER,
    filtro_sub_categoria INTEGER,
    filtro_proveedor INTEGER,
    affected_count INTEGER NOT NULL DEFAULT 0,
    estado TEXT NOT NULL DEFAULT 'aplicada' CHECK (estado IN ('aplicada', 'deshecha')),
    created_at TEXT NOT NULL,
    undone_at TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (filtro_categoria) REFERENCES categorias(id),
    FOREIGN KEY (filtro_sub_categoria) REFERENCES sub_categorias(id),
    FOREIGN KEY (filtro_proveedor) REFERENCES proveedores(id)
);
Tabla cost_update_items (detalle)
CREATE TABLE IF NOT EXISTS cost_update_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    operation_id INTEGER NOT NULL,
    id_stock INTEGER NOT NULL,
    costo_anterior REAL NOT NULL,
    costo_nuevo REAL NOT NULL,
    FOREIGN KEY (operation_id) REFERENCES cost_update_operations(id) ON DELETE CASCADE,
    FOREIGN KEY (id_stock) REFERENCES stock(id)
);
Índices
CREATE INDEX IF NOT EXISTS idx_cost_update_operations_estado ON cost_update_operations(estado);
CREATE INDEX IF NOT EXISTS idx_cost_update_operations_created_at ON cost_update_operations(created_at);
CREATE INDEX IF NOT EXISTS idx_cost_update_items_operation_id ON cost_update_items(operation_id);
Tamaño estimado
Cada operación bulk de N artículos genera:
- 1 fila en cost_update_operations (~200 bytes)
- N filas en cost_update_items (~60 bytes cada una)
Para 1000 artículos: ~60 KB por operación. Para 100 operaciones al año: ~6 MB. Despreciable.
No se requiere purga automática. Si en el futuro se necesita, se puede agregar retención por tiempo (similar a audit_logs).
13. Almacenamiento del filtro
Sí almacenar los filtros en la cabecera. Razones:
1. Auditoría: permite responder "¿qué filtros usó el usuario en la operación #152?"
2. Explicación: en el frontend se puede mostrar: "Operación: categoría=Bebidas, proveedor=X, +20%"
3. Debugging: si hay un problema, se puede reproducir la operación
4. Costo mínimo: 3 columnas NULLable en la cabecera
5. No duplica información: los filtros son metadatos de la operación, no datos del stock
Los filtros se almacenan como IDs (FK). El frontend puede hacer JOIN para mostrar los nombres.
14. Comportamiento del frontend
Botón de Undo
Ubicación recomendada: en la página ActualizarCostoPage.vue, junto a los botones de "Vista Previa" y "Aplicar Cambios" en la actions-bar.
<div class="actions-bar">
    <button @click="loadPreview">Vista Previa</button>
    <button v-if="tienePreview && !applied" @click="handleApply">Aplicar Cambios</button>
    <span v-if="applied">Cambios aplicados correctamente.</span>
    
    <button 
        v-if="lastOperation" 
        class="btn-secondary" 
        @click="handleUndo"
    >
        Deshacer última operación
    </button>
</div>
Cuándo mostrar el botón de Undo
- Cargar lastOperation al montar la página.
- Mostrar el botón solo si existe una operación con estado 'aplicada'.
- Si no hay operaciones para deshacer, ocultar el botón.
Información a mostrar
Deshacer última operación: +20% sobre 125 artículos (Categoría: Bebidas)
Se compone con los datos de la cabecera: porcentaje, affected_count, y los nombres de las categorías/subcategorías/proveedores filtrados (resueltos via JOIN en el query).
Confirmación
Usar useConfirm() con variante danger:
¿Está seguro de deshacer la última actualización de precios?
Se restaurarán los valores anteriores de 125 artículos.
Resultado
- Toast de éxito: "Se deshizo la operación: 125 artículos restaurados."
- Si hay error (artículo modificado después): mostrar error.
- Refrescar stock después del undo.
15. Historial futuro
La arquitectura propuesta (cabecera + detalle + estados) permite naturalmente:
1. Ver historial de operaciones: consultar cost_update_operations con paginación.
2. Deshacer operación específica: cambiar el parámetro de "última" a "operación ID específica".
3. Rehacer: crear una nueva operación con el porcentaje original, usando los datos de los items como referencia.
No se implementa ahora, pero el modelo lo permite sin refactorizaciones.
16. Auditoría
Información que ya existe
- audit_logs: user_id, username, screen, action, detail, created_at
- Detail de la operación bulk actual: "Actualización masiva de costos: +20% (125 artículos)"
Información que se agrega con el historial
- Cabecera de operación: user_id, porcentaje, filtros, affected_count, estado, timestamps
- Detalle: id_stock, costo_anterior, costo_nuevo por cada artículo
- El undo genera una entrada adicional en audit_logs con action=Update y detail: "Deshacer actualización de costos (operación #152): 125 artículos restaurados"
Auditoría doble
Se mantiene la escritura en audit_logs (existente) y se registra en las tablas nuevas. Son complementarias:
- audit_logs → registro rápido y unificado de todas las acciones del sistema
- cost_update_operations → historial detallado específico de actualizaciones de costo con datos suficientes para undo
17. Tests
Tests unitarios (entidades)
- CostUpdateOperation: serialización JSON, campos por defecto
- CostUpdateItem: serialización JSON
Tests de repositorio
- create_operation: inserta cabecera, verifica campos
- create_items: inserta items vinculados a una operación
- find_last_undoable: retorna la operación más reciente con estado 'aplicada'
- find_last_undoable (vacío): retorna None cuando no hay operaciones
- find_last_undoable (todas deshechas): retorna None
- undo_operation: cambia estado a 'deshecha', setea undone_at
- verify_no_posterior_modifications: compara costo_nuevo del item con costo actual del stock
Tests de servicio
- apply_costo_percentage con historial: crea operación + items + actualiza stock en transacción
- undo_operation exitoso: restaura valores, marca estado
- undo_operation con artículo modificado posteriormente: retorna error
- undo_operation sobre operación ya deshecha: retorna error
- undo_operation sobre operación inexistente: retorna error
- get_last_undoable con datos: retorna operación correcta
- get_last_undoable sin datos: retorna None
Tests de integridad
- Verificar que costo_anterior del item == costo del stock antes del UPDATE
- Verificar que costo_nuevo del item == costo del stock después del UPDATE
- Verificar que después del undo, el costo del stock == costo_anterior original
- Verificar atomicidad: si falla la inserción de items, el stock no se modifica
18. Comparación de alternativas
Alternativa	Ventajas	Desventajas	Riesgos	Recomendación
Invertir porcentaje	Sin tablas nuevas, mínima implementación	Redondeos imprecisos, no funciona con modificaciones manuales, no escala a múltiples operaciones	Pérdida de integridad de datos, valores incorrectos tras undo	Descartada
Guardar valor anterior (sin cabecera)	Valor exacto almacenado, restore preciso	Sin agrupación lógica, no se puede deshacer "la última operación" como unidad, no escala a historial	Imposible determinar qué items pertenecen a la misma operación	Insuficiente
Historial de operaciones (cabecera + detalle)	Restore exacto, operación atómica lógica, auditoría completa, extensible, no depende de cálculos	2 tablas nuevas, más complejidad en la transacción, más almacenamiento	Bajo: el patrón es estándar, SQLite lo soporta bien, el mutex serializa concurrencia	Recomendada
19. Propuesta final
Modelo de datos
Tablas nuevas:
- cost_update_operations (cabecera) — 10 columnas
- cost_update_items (detalle) — 5 columnas
- 3 índices nuevos
No se modifican tablas existentes.
Casos de uso
1. GetCostUpdatePreview (extiende el existente)
2. ApplyCostoPercentage (extiende el existente, agrega transacción con historial)
3. GetLastUndoableOperation (nuevo)
4. UndoCostUpdateOperation (nuevo)
Repositories
- CostUpdateRepository (trait nuevo en domain)
- create_operation(&self, op) -> CostUpdateOperation
- create_items(&self, items: Vec<CostUpdateItem>)
- find_last_undoable(&self) -> Option<CostUpdateOperation>
- find_items_by_operation(&self, operation_id) -> Vec<CostUpdateItem>
- mark_as_undone(&self, operation_id)
- update_affected_count(&self, operation_id, count)
- Extensión de StockRepository:
- apply_costo_percentage_in_transaction(porcentaje, filtros) -> Result<(i64, Vec<StockCostSnapshot>)> — retorna count + snapshots para insertar en items
DTOs
// Request existente, sin cambios
ApplyCostoPercentageRequest { porcentaje, id_categoria, id_sub_categoria, id_proveedor }

// Resultado existente, sin cambios
ApplyCostoPercentageResult { updated_count }

// Nuevos
UndoOperationResult { restored_count: i64 }
LastOperationResponse { id, porcentaje, affected_count, categoria_nombre, sub_categoria_nombre, proveedor_nombre, created_at, estado }
Commands de Tauri
Command	Descripción
get_stock_preview_costo	(existente, sin cambios)
apply_costo_percentage_stock	(existente, modificado para crear historial)
get_last_undoable_cost_update	(nuevo) — retorna la última operación deshacible
undo_cost_update	(nuevo) — deshace la última operación
Permisos
Reutilizar PermissionCode::UpdateStock (modificar_stock) para undo. Quien puede modificar precios, puede deshacer. No se crea un permiso nuevo.
Frontend — Archivos a modificar
Archivo	Cambios
src/domain/entities/types.ts	Agregar interfaces CostUpdateOperation, LastOperationResponse, UndoOperationResult
src/domain/interfaces/stockRepository.ts	Agregar getLastUndoableCostUpdate(), undoCostUpdate(operationId)
src/infrastructure/api/stockRepository.ts	Implementar los 2 nuevos invokes
src/application/usecases/stock.ts	Agregar 2 delegaciones nuevas
src/presentation/stores/stockStore.ts	Agregar 2 acciones nuevas + lastOperation state
src/presentation/pages/ActualizarCostoPage.vue	Agregar botón de Undo + lógica de carga
Frontend — Archivos nuevos
Ninguno. Todo se integra en los archivos existentes.
Backend — Archivos nuevos
Archivo	Contenido
src-tauri/src/domain/entities/cost_update_operation.rs	Entidad CostUpdateOperation
src-tauri/src/domain/entities/cost_update_item.rs	Entidad CostUpdateItem
src-tauri/src/domain/repositories/cost_update_repository.rs	Trait CostUpdateRepository
src-tauri/src/infrastructure/repositories/cost_update_repository.rs	Implementación SQLite
Backend — Archivos a modificar
Archivo	Cambios
domain/entities/mod.rs	Registrar módulos nuevos
domain/repositories/mod.rs	Registrar trait nuevo
infrastructure/repositories/mod.rs	Registrar implementación nueva
infrastructure/error.rs	Agregar variantes CostUpdateModifiedAfter, CostUpdateAlreadyUndone, CostUpdateNotFound
application/services/stock_service.rs	Modificar apply_costo_percentage para crear historial + transacción
api/commands/stock_commands.rs	Modificar apply_costo_percentage_stock, agregar 2 commands nuevos
api/commands/mod.rs	Exportar commands nuevos
lib.rs	Registrar .manage(CostUpdateAppState) + commands
infrastructure/database/mod.rs	Agregar CREATE TABLE para las 2 tablas nuevas + 3 índices
Flujo de actualización (modificado)
Usuario → Aplicar Cambios
  ↓
Validar porcentaje
  ↓
BEGIN TRANSACTION
  ├─ 1. SELECT stock.id, stock.costo WHERE filtros  (para obtener snapshots)
  ├─ 2. INSERT INTO cost_update_operations (cabecera)
  ├─ 3. INSERT INTO cost_update_items (items con costo_anterior + costo_nuevo calculado)
  ├─ 4. UPDATE stock SET costo = ROUND(costo * (1 + %/100), 2) WHERE filtros
  ├─ 5. UPDATE cost_update_operations SET affected_count = COUNT(items)
  └─ COMMIT
  ↓
Retornar { updated_count }
Flujo de Undo
Usuario → Deshacer última operación
  ↓
BEGIN TRANSACTION
  ├─ 1. Buscar última operación con estado = 'aplicada'
  ├─ 2. Verificar que no haya artículos modificados después:
  │     SELECT COUNT(*) FROM cost_update_items item
  │     JOIN stock ON item.id_stock = stock.id
  │     WHERE item.operation_id = ? AND stock.costo != item.costo_nuevo
  ├─ 3. Si hay modificaciones → ROLLBACK, error CostUpdateModifiedAfter
  ├─ 4. Restaurar: UPDATE stock SET costo = (
  │       SELECT item.costo_anterior FROM cost_update_items item
  │       WHERE item.id_stock = stock.id AND item.operation_id = ?
  │     ) WHERE id IN (items de la operación)
  ├─ 5. UPDATE cost_update_operations SET estado = 'deshecha', undone_at = datetime('now')
  ├─ 6. log_audit(user_id, Stock, Update, "Deshacer operación #152: 125 artículos")
  └─ COMMIT
  ↓
Retornar { restored_count: 125 }
Estrategia transaccional
Todas las operaciones de escritura (apply + undo) ocurren dentro de una única transacción SQLite. El Mutex<StockService> serializa el acceso a nivel de Tauri.
Manejo de errores
Error	Código	Mensaje
Porcentaje inválido	bulk_update_invalid_porcentaje	"El porcentaje ingresado es inválido."
Sin matches	bulk_update_no_matches	"No se encontraron artículos con los filtros seleccionados."
Operación no encontrada	cost_update_not_found	"No se encontró la operación de actualización."
Ya deshecha	cost_update_already_undone	"La operación ya fue deshecha."
Artículo modificado después	cost_update_modified_after	"No se puede deshacer: {n} artículo(s) fueron modificados después de la operación."
DB error	database_error	"Ocurrió un error en la base de datos."
Validaciones
Apply:
- Porcentaje: finite, ≠ 0, > -100, ≤ 100
- Al menos 1 artículo debe coincidir con los filtros
Undo:
- Debe existir una operación con estado 'aplicada'
- Ningún artículo de la operación puede haber sido modificado después
- Transacción atómica
Estrategia de testing
- 16+ tests nuevos (unitarios de entidades + integración de repositorio + tests de servicio)
- Verificar atomicidad: si falla la inserción de items, el stock no se modifica
- Verificar restore exacto: después de undo, costo == costo_anterior original
- Verificar que operaciones deshechas no se pueden deshacer de nuevo
- Verificar que operaciones con artículos modificados post-op son bloqueadas
Archivos a crear (resumen)
Archivo	Capa
src-tauri/src/domain/entities/cost_update_operation.rs	Domain
src-tauri/src/domain/entities/cost_update_item.rs	Domain
src-tauri/src/domain/repositories/cost_update_repository.rs	Domain
src-tauri/src/infrastructure/repositories/cost_update_repository.rs	Infrastructure
Archivos a modificar (resumen)
Archivo	Capa
src-tauri/src/domain/entities/mod.rs	Domain
src-tauri/src/domain/repositories/mod.rs	Domain
src-tauri/src/infrastructure/error.rs	Infrastructure
src-tauri/src/infrastructure/repositories/mod.rs	Infrastructure
src-tauri/src/infrastructure/database/mod.rs	Infrastructure
src-tauri/src/application/services/stock_service.rs	Application
src-tauri/src/api/commands/stock_commands.rs	API
src-tauri/src/api/commands/mod.rs	API
src-tauri/src/lib.rs	API
src/domain/entities/types.ts	Frontend Domain
src/domain/interfaces/stockRepository.ts	Frontend Domain
src/infrastructure/api/stockRepository.ts	Frontend Infrastructure
src/application/usecases/stock.ts	Frontend Application
src/presentation/stores/stockStore.ts	Frontend Presentation
src/presentation/pages/ActualizarCostoPage.vue	Frontend Presentation
20. Información faltante
No encontré información faltante en el proyecto. Todo lo necesario para tomar decisiones de diseño está disponible en el codebase:
- Esquema completo de la DB
- Patrón de auditoría existente
- Arquitectura de capas
- Patrón de commands y states de Tauri
- Sistema de permisos
- Convenciones de código (Rust y TypeScript)
No hay necesidad de inventar datos. La propuesta se basa exclusivamente en la estructura real del proyecto.
Respuesta a la pregunta final
¿Cómo podemos implementar un mecanismo confiable para deshacer las últimas modificaciones de precios de costo sin depender de cálculos matemáticos inversos y sin poner en riesgo la integridad de los datos?
Mediante una arquitectura de historial de operaciones con cabecera + detalle, donde cada operación bulk de actualización de precios:
1. Captura los valores anteriores reales de cada artículo afectado antes de modificarlos (almacenados en cost_update_items.costo_anterior).
2. Agrupa los cambios en una operación lógica (almacenada en cost_update_operations).
3. Garantiza atomicidad mediante una única transacción SQLite que incluye la creación del historial y la actualización del stock.
4. Permite undo seguro verificando que ningún artículo fue modificado post-operación antes de restaurar.
5. No depende de cálculos inversos — restaura el valor real registrado.
6. Extiende el sistema de auditoría existente sin reemplazarlo.
La solución es coherente con Clean Architecture (capas separadas, Dependency Inversion), SOLID (SRP con CostUpdateService dedicado), y es extensible para funcionalidades futuras (historial visual, rehacer, deshacer específica).