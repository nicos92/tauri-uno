<script setup lang="ts">
import { ref, computed, onMounted, nextTick, watch } from "vue";
import { useRouter } from "vue-router";
import {
  useVentasStore,
  useStockStore,
  useArticulosStore,
  useTiposVentaStore,
  useClientesStore,
} from "../stores";
import { usePermissions } from "../composables/usePermissions";
import { useToasts } from "../composables/useToasts";
import { formatMoney } from "../utils/format";
import type {
  Cliente,
  CreateClienteRequest,
  CreateVentaRequest,
} from "../../domain/entities";

const router = useRouter();
const ventasStore = useVentasStore();
const stockStore = useStockStore();
const articulosStore = useArticulosStore();
const tiposVentaStore = useTiposVentaStore();
const clientesStore = useClientesStore();
const {
  canVenderSinStock,
  canGenerarPresupuesto,
  canViewClientes,
  canCreateCliente,
} = usePermissions();
const { error: toastError, success: toastSuccess } = useToasts();

interface CartItem {
  id_articulo: number;
  cod_articulo: string;
  articulo: string;
  stockDisponible: number;
  cantidad: number;
  precio: number;
  subtotal: number;
}

interface StockArticulo {
  id_articulo: number;
  cod_articulo: string;
  articulo: string;
  stockDisponible: number;
  precioVenta: number;
}

const searchQuery = ref("");
const searchInput = ref<HTMLInputElement | null>(null);
const observacion = ref("");
const descuento = ref<number>(0);
const cart = ref<CartItem[]>([]);
const tipoVentaId = ref<number | null>(null);

const clienteSeleccionado = ref<Cliente | null>(null);
const clienteDefecto = ref<Cliente | null>(null);
const clienteQuery = ref("");
const mostrandoClientes = ref(false);
const mostrarModalNuevoCliente = ref(false);

const nuevoNombre = ref("");
const nuevoApellido = ref("");
const nuevoTelefono = ref("");
const nuevoEmail = ref("");
const nuevoDireccion = ref("");

const nuevoClienteInvalido = computed(
  () =>
    !nuevoNombre.value.trim() &&
    !nuevoApellido.value.trim() &&
    !nuevoTelefono.value.trim() &&
    !nuevoEmail.value.trim() &&
    !nuevoDireccion.value.trim(),
);

function clienteLabel(cliente: Cliente): string {
  const name = [cliente.nombre, cliente.apellido]
    .filter(Boolean)
    .join(" ");
  return name || cliente.telefono || cliente.email || "Sin datos";
}

const clientesFiltrados = computed<Cliente[]>(() => {
  const query = clienteQuery.value.trim().toLowerCase();
  const base = clientesStore.clientes;
  if (!query) return base;
  return base.filter(
    (c) =>
      (c.nombre || "").toLowerCase().includes(query) ||
      (c.apellido || "").toLowerCase().includes(query) ||
      (c.telefono || "").toLowerCase().includes(query) ||
      (c.email || "").toLowerCase().includes(query),
  );
});

watch(
    () => tiposVentaStore.tipos,
    (tipos) => {
        if (tipoVentaId.value === null && tipos.length > 0) {
            const efectivo = tipos.find((t) => t.nombre === "Efectivo");
            tipoVentaId.value = efectivo ? efectivo.id : tipos[0].id;
        }
    },
    { immediate: true },
);

const articulosVendibles = computed<StockArticulo[]>(() => {
  return stockStore.stocks.map((s) => {
    const articulo = articulosStore.articulos.find(
      (a) => a.id === s.id_articulo,
    );
    return {
      id_articulo: s.id_articulo,
      cod_articulo: articulo?.cod_articulo || "-",
      articulo: articulo?.articulo || "Sin artículo",
      stockDisponible: s.cantidad,
      precioVenta: stockStore.calcularPrecioVenta(s.costo, s.ganancia),
    };
  });
});

const searchResults = computed<StockArticulo[]>(() => {
  const query = searchQuery.value.trim().toLowerCase();
  const inCart = new Set(cart.value.map((c) => c.id_articulo));
  const base = articulosVendibles.value.filter((a) => !inCart.has(a.id_articulo));
  if (!query) return [];
  return base
    .filter(
      (a) =>
        a.cod_articulo.toLowerCase().includes(query) ||
        a.articulo.toLowerCase().includes(query),
    )
    .slice(0, 20);
});

const carritoSubtotal = computed(() =>
  cart.value.reduce((acc, item) => acc + item.subtotal, 0),
);

const descuentoMonto = computed(() => {
  const d = Number.isFinite(descuento.value) ? descuento.value : 0;
  return (carritoSubtotal.value * d) / 100;
});

const carritoTotal = computed(() => carritoSubtotal.value - descuentoMonto.value);

const descuentoValido = computed(
  () =>
    (descuento.value === null ||
      (descuento.value >= 0 && descuento.value <= 100)),
);

const carritoValido = computed(
  () =>
    cart.value.length > 0 &&
    descuentoValido.value &&
    tipoVentaId.value !== null &&
    cart.value.every((i) => i.cantidad > 0 && i.precio >= 0),
);

const fechaHoy = computed(() => new Date().toLocaleDateString());

onMounted(async () => {
  await Promise.all([
    stockStore.fetchStock(),
    articulosStore.fetchArticulos(),
    tiposVentaStore.fetchTiposVenta(),
    canViewClientes() ? clientesStore.fetchClientes() : Promise.resolve(),
  ]);
  await ventasStore.checkDiaCerrado();
  if (canViewClientes()) {
    const clienteDef = await clientesStore.getClienteDefecto();
    clienteDefecto.value = clienteDef;
    clienteSeleccionado.value = clienteDef;
  }
  focusSearch();
});

function focusSearch() {
  nextTick(() => {
    searchInput.value?.focus();
  });
}

function addArticuloById(idArticulo: number) {
  const articulo = articulosVendibles.value.find(
    (a) => a.id_articulo === idArticulo,
  );
  if (!articulo) return;

  const existing = cart.value.find((c) => c.id_articulo === idArticulo);
  if (existing) {
    existing.cantidad += 1;
    existing.subtotal = existing.cantidad * existing.precio;
  } else {
    cart.value.push({
      id_articulo: articulo.id_articulo,
      cod_articulo: articulo.cod_articulo,
      articulo: articulo.articulo,
      stockDisponible: articulo.stockDisponible,
      cantidad: 1,
      precio: articulo.precioVenta,
      subtotal: articulo.precioVenta,
    });
  }

  searchQuery.value = "";
  focusSearch();
}

function onSearchEnter() {
  const query = searchQuery.value.trim().toLowerCase();
  if (!query) return;
  const exact = articulosVendibles.value.find(
    (a) => a.cod_articulo.toLowerCase() === query,
  );
  if (exact) {
    addArticuloById(exact.id_articulo);
    return;
  }
  if (searchResults.value.length === 1) {
    addArticuloById(searchResults.value[0].id_articulo);
  }
}

function removeArticulo(idArticulo: number) {
  cart.value = cart.value.filter((c) => c.id_articulo !== idArticulo);
}

function vaciarCarrito() {
  cart.value = [];
}

function updateSubtotal(item: CartItem) {
  item.subtotal = item.cantidad * item.precio;
}

function stockWarning(item: CartItem): boolean {
  return (
    item.cantidad > item.stockDisponible && !canVenderSinStock()
  );
}

function resetForm() {
  cart.value = [];
  descuento.value = 0;
  observacion.value = "";
  searchQuery.value = "";
  const efectivo = tiposVentaStore.tipos.find((t) => t.nombre === "Efectivo");
  tipoVentaId.value = efectivo
    ? efectivo.id
    : tiposVentaStore.tipos[0]?.id ?? null;
  clienteSeleccionado.value = clienteDefecto.value;
  clienteQuery.value = "";
  mostrandoClientes.value = false;
}

function onClienteBlur() {
  setTimeout(() => {
    mostrandoClientes.value = false;
  }, 150);
}

function seleccionarCliente(cliente: Cliente) {
  clienteSeleccionado.value = cliente;
  clienteQuery.value = "";
  mostrandoClientes.value = false;
}

function quitarCliente() {
  clienteSeleccionado.value = clienteDefecto.value;
  clienteQuery.value = "";
  mostrandoClientes.value = false;
}

function abrirModalNuevoCliente() {
  nuevoNombre.value = "";
  nuevoApellido.value = "";
  nuevoTelefono.value = "";
  nuevoEmail.value = "";
  nuevoDireccion.value = "";
  clientesStore.error = null;
  mostrarModalNuevoCliente.value = true;
}

async function crearClienteRapido() {
  if (nuevoClienteInvalido.value) return;
  const request: CreateClienteRequest = {
    nombre: nuevoNombre.value.trim() || undefined,
    apellido: nuevoApellido.value.trim() || undefined,
    telefono: nuevoTelefono.value.trim() || undefined,
    email: nuevoEmail.value.trim() || undefined,
    direccion: nuevoDireccion.value.trim() || undefined,
  };
  const nuevoCliente = await clientesStore.crearCliente(request);
  if (nuevoCliente) {
    seleccionarCliente(nuevoCliente);
    mostrarModalNuevoCliente.value = false;
  }
}

async function handleCreate() {
  if (!carritoValido.value) return;
  const request: CreateVentaRequest = {
    items: cart.value.map((item) => ({
      id_articulo: item.id_articulo,
      cantidad: item.cantidad,
      precio_unitario: item.precio,
    })),
    descuento: descuento.value || 0,
    observacion: observacion.value.trim() || undefined,
    id_tipo_venta: tipoVentaId.value || undefined,
    cliente_id: clienteSeleccionado.value?.id,
  };
  const venta = await ventasStore.createVenta(request);
  if (venta) {
    toastSuccess(`Venta N° ${venta.id} registrada.`);
    resetForm();
    focusSearch();
  } else {
    toastError(ventasStore.error || "No se pudo registrar la venta.");
  }
}

function cancelar() {
  router.push({ name: "ventas" });
}

function generarPdf() {
  if (cart.value.length === 0) return;
  window.print();
}
</script>

<template>
    <div class="nueva-venta-page">
        <div class="page-header">
            <div class="header-left">
                <button
                    type="button"
                    @click="cancelar"
                    class="btn-secondary btn-volver"
                >
                    ← Volver
                </button>
                <h1>Nueva Venta</h1>
            </div>
        </div>

        <div v-if="ventasStore.diaCerrado" class="dia-cerrado-banner">
            Día cerrado, no se pueden ingresar más ventas.
        </div>

        <div class="venta-section header-section">
            <div class="form-group obs-group">
                <label>Observación</label>
                <input
                    v-model="observacion"
                    type="text"
                    placeholder="Opcional"
                />
            </div>
            <div class="form-group obs-group">
                <label>Tipo de venta</label>
                <select v-model.number="tipoVentaId" class="tipo-select">
                    <option
                        v-for="tipo in tiposVentaStore.tipos"
                        :key="tipo.id"
                        :value="tipo.id"
                    >
                        {{ tipo.nombre }}
                    </option>
                </select>
            </div>
           
            <div class="acciones">
                <button
                    v-if="canGenerarPresupuesto()"
                    type="button"
                    @click="generarPdf"
                    class="btn-tertiary"
                    :disabled="cart.length === 0"
                >
                    PDF
                </button>
                <button
                    type="button"
                    @click="cancelar"
                    class="btn-secondary"
                >
                    Cancelar
                </button>
                <button
                    type="button"
                    @click="handleCreate"
                    class="btn-primary"
                    :disabled="!carritoValido || ventasStore.diaCerrado"
                >
                    Registrar Venta
                </button>
            </div>
        </div>

        <div class="venta-section totals-section">
             <div v-if="canViewClientes()" class="form-group obs-group cliente-group">
                <label>Cliente</label>
                <div class="cliente-selector">
                    <input
                        v-model="clienteQuery"
                        type="text"
                        placeholder="Buscar por nombre, apellido o teléfono..."
                        @focus="mostrandoClientes = true"
                        @input="mostrandoClientes = true"
                        @blur="onClienteBlur"
                    />
                    <button
                        v-if="canCreateCliente()"
                        type="button"
                        class="btn-nuevo-cliente"
                        title="Crear nuevo cliente"
                        @click="abrirModalNuevoCliente"
                    >
                        + Nuevo
                    </button>
                </div>
                <div v-if="mostrandoClientes" class="cliente-dropdown">
                    <button
                        v-for="cliente in clientesFiltrados"
                        :key="cliente.id"
                        type="button"
                        class="cliente-option"
                        @mousedown.prevent="seleccionarCliente(cliente)"
                    >
                        <span class="cliente-nombre">
                            {{ clienteLabel(cliente) }}
                        </span>
                        <span v-if="cliente.telefono" class="cliente-tel">
                            {{ cliente.telefono }}
                        </span>
                    </button>
                    <div v-if="clientesFiltrados.length === 0" class="cliente-empty">
                        Sin coincidencias
                    </div>
                </div>
                <div
                    v-if="clienteSeleccionado && !mostrandoClientes"
                    class="cliente-seleccionado"
                >
                    <span>{{ clienteLabel(clienteSeleccionado) }}</span>
                    <button
                        type="button"
                        class="cliente-limpiar"
                        title="Usar Consumidor Final"
                        @click="quitarCliente"
                    >
                        ×
                    </button>
                </div>
            </div>
            <div class="total-box">
                <span class="total-label">Subtotal</span>
                <span class="total-value">{{ formatMoney(carritoSubtotal) }}</span>
            </div>
            <div class="total-box">
                <span class="total-label">Descuento</span>
                <div class="descuento-row">
                    <input
                        v-model.number="descuento"
                        type="number"
                        step="0.01"
                        min="0"
                        max="100"
                        class="descuento-input"
                    />
                    <span>%</span>
                    <span v-if="descuentoMonto > 0" class="descuento-monto">
                        −{{ formatMoney(descuentoMonto) }}
                    </span>
                </div>
            </div>
            <div class="total-box total-final">
                <span class="total-label">Total</span>
                <span class="total-value">{{ formatMoney(carritoTotal) }}</span>
            </div>
        </div>

        <div class="venta-section search-section">
            <div class="cart-header">
                <h2>Artículos de la venta</h2>
                <button
                    v-if="cart.length > 0"
                    type="button"
                    @click="vaciarCarrito"
                    class="btn-secondary"
                >
                    Vaciar
                </button>
            </div>
            <div class="form-group">
                <label>Buscar artículo por código o nombre</label>
                <input
                    ref="searchInput"
                    v-model="searchQuery"
                    type="text"
                    placeholder="Escriba el código y presione Enter..."
                    @keydown.enter.prevent="onSearchEnter"
                />
            </div>
            <div v-if="searchResults.length > 0" class="search-results">
                <button
                    v-for="result in searchResults"
                    :key="result.id_articulo"
                    type="button"
                    @click="addArticuloById(result.id_articulo)"
                    class="search-result-item"
                >
                    <span class="result-code">{{ result.cod_articulo }}</span>
                    <span class="result-name">{{ result.articulo }}</span>
                    <span class="result-stock">
                        Stock: {{ result.stockDisponible }}
                    </span>
                    <span class="result-precio">
                        {{ formatMoney(result.precioVenta) }}
                    </span>
                </button>
            </div>
            <div
                v-if="searchQuery.trim() && searchResults.length === 0"
                class="empty-state small"
            >
                Sin coincidencias
            </div>

            

            <table v-if="cart.length > 0" class="cart-table">
                <thead>
                    <tr>
                        <th>Código</th>
                        <th>Artículo</th>
                        <th>Cantidad</th>
                        <th>Precio</th>
                        <th>Subtotal</th>
                        <th>Acciones</th>
                    </tr>
                </thead>
                <tbody>
                    <tr
                        v-for="item in cart"
                        :key="item.id_articulo"
                        :class="{ 'stock-warning-row': stockWarning(item) }"
                    >
                        <td>{{ item.cod_articulo }}</td>
                        <td>
                            {{ item.articulo }}
                            <span
                                v-if="stockWarning(item)"
                                class="stock-warning"
                            >
                                ⚠ Stock insuficiente ({{ item.stockDisponible }})
                            </span>
                        </td>
                        <td>
                            <input
                                v-model.number="item.cantidad"
                                type="number"
                                step="0.01"
                                min="0.01"
                                @input="updateSubtotal(item)"
                                class="cart-input"
                            />
                        </td>
                        <td>
                            <input
                                v-model.number="item.precio"
                                type="number"
                                step="0.01"
                                min="0"
                                @input="updateSubtotal(item)"
                                class="cart-input"
                            />
                        </td>
                        <td>{{ formatMoney(item.subtotal) }}</td>
                        <td>
                            <button
                                @click="removeArticulo(item.id_articulo)"
                                class="btn-icon btn-danger"
                                title="Quitar"
                            >
                                <img src="/svg/trash.svg" alt="Quitar" />
                            </button>
                        </td>
                    </tr>
                </tbody>
            </table>

            <div v-if="cart.length === 0" class="empty-state">
                Agregue artículos a la venta
            </div>
        </div>

        <div v-if="ventasStore.error" class="error-banner">
            {{ ventasStore.error }}
        </div>

        <div
            v-if="mostrarModalNuevoCliente"
            class="modal-overlay"
            @click.self="mostrarModalNuevoCliente = false"
        >
            <div class="modal-card">
                <div class="modal-header">
                    <h2>Nuevo cliente</h2>
                    <button
                        type="button"
                        class="btn-icon"
                        @click="mostrarModalNuevoCliente = false"
                    >
                        ×
                    </button>
                </div>
                <form @submit.prevent="crearClienteRapido">
                    <div class="form-group">
                        <label>Nombre</label>
                        <input v-model="nuevoNombre" type="text" placeholder="Nombre" />
                    </div>
                    <div class="form-group">
                        <label>Apellido</label>
                        <input v-model="nuevoApellido" type="text" placeholder="Apellido" />
                    </div>
                    <div class="form-group">
                        <label>Teléfono</label>
                        <input v-model="nuevoTelefono" type="text" placeholder="Teléfono" />
                    </div>
                    <div class="form-group">
                        <label>Email</label>
                        <input v-model="nuevoEmail" type="email" placeholder="Email" />
                    </div>
                    <div class="form-group">
                        <label>Dirección</label>
                        <input v-model="nuevoDireccion" type="text" placeholder="Dirección" />
                    </div>
                    <div v-if="clientesStore.error" class="error-text">
                        {{ clientesStore.error }}
                    </div>
                    <div class="modal-actions">
                        <button
                            type="button"
                            class="btn-secondary"
                            @click="mostrarModalNuevoCliente = false"
                        >
                            Cancelar
                        </button>
                        <button
                            type="submit"
                            class="btn-primary"
                            :disabled="nuevoClienteInvalido"
                        >
                            Crear y seleccionar
                        </button>
                    </div>
                </form>
            </div>
        </div>
    </div>

    <Teleport to="body">
        <div class="print-area" id="print-area">
            <h1>Presupuesto</h1>
            <p>Fecha: {{ fechaHoy }}</p>
            <p v-if="clienteSeleccionado">
                Cliente: {{ clienteLabel(clienteSeleccionado) }}
            </p>
            <div class="print-summary">
                <p class="print-line">Subtotal: {{ formatMoney(carritoSubtotal) }}</p>
                <p v-if="descuento > 0" class="print-line">
                    Descuento ({{ descuento }}%): −{{ formatMoney(descuentoMonto) }}
                </p>
                <p class="print-total">Total: {{ formatMoney(carritoTotal) }}</p>
                <p v-if="observacion" class="print-obs">
                    Observación: {{ observacion }}
                </p>
            </div>
            <table>
                <thead>
                    <tr>
                        <th>Código</th>
                        <th>Artículo</th>
                        <th>Cantidad</th>
                        <th>Precio</th>
                        <th>Subtotal</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="item in cart" :key="item.id_articulo">
                        <td>{{ item.cod_articulo }}</td>
                        <td>{{ item.articulo }}</td>
                        <td>{{ item.cantidad }}</td>
                        <td>{{ formatMoney(item.precio) }}</td>
                        <td>{{ formatMoney(item.subtotal) }}</td>
                    </tr>
                </tbody>
            </table>
        </div>
    </Teleport>
</template>

<style scoped>
.nueva-venta-page {
    padding: 1rem;
    background: var(--color-bg);
    min-height: 100%;
}

.page-header {
    margin-bottom: 1rem;
}

.header-left {
    display: flex;
    align-items: center;
    gap: 1rem;
}

.header-left h1 {
    margin: 0;
}

.dia-cerrado-banner {
    color: #e53e3e;
    background: rgba(229, 62, 62, 0.1);
    border: 1px solid rgba(229, 62, 62, 0.3);
    padding: 0.75rem 1rem;
    border-radius: 6px;
    margin-bottom: 1rem;
    font-weight: 600;
}

.venta-section {
    background: var(--color-surface);
    border-radius: 12px;
    padding: 1.5rem;
    margin-bottom: 1rem;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.header-section {
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    gap: 1rem;
    flex-wrap: wrap;
}

.obs-group {
    flex: 1;
    min-width: 240px;
    margin: 0;
}

.acciones {
    display: flex;
    gap: 0.75rem;
}

.totals-section {
    display: flex;
    justify-content: space-between;
    gap: 1rem;
    flex-wrap: wrap;
}

.total-box {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: 1rem;
    border-radius: 8px;
    background: var(--color-surface-2);
    min-width: 180px;
}

.total-final {
    background: #2D195D;
    color: white;
}

.total-label {
    font-size: 0.85rem;
    text-transform: uppercase;
    opacity: 0.8;
}

.total-value {
    font-size: 1.4rem;
    font-weight: 700;
}

.descuento-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 1.1rem;
    font-weight: 600;
}

.descuento-input {
    width: 80px;
    padding: 0.5rem;
    border: 1px solid var(--color-border);
    border-radius: 6px;
    background: var(--color-surface);
    color: var(--color-text);
}

.descuento-monto {
    color: #e53e3e;
    font-size: 0.9rem;
}

.search-section .form-group {
    margin-bottom: 0.75rem;
}

.search-results {
    border: 1px solid var(--color-border);
    border-radius: 8px;
    overflow: hidden;
}

.search-result-item {
    display: flex;
    align-items: center;
    gap: 1rem;
    width: 100%;
    padding: 0.75rem 1rem;
    background: var(--color-surface);
    border: none;
    border-bottom: 1px solid var(--color-border);
    cursor: pointer;
    text-align: left;
    color: var(--color-text);
}

.search-result-item:last-child {
    border-bottom: none;
}

.search-result-item:hover {
    background: var(--color-surface-2);
}

.result-code {
    font-weight: 600;
    min-width: 110px;
}

.result-name {
    flex: 1;
}

.result-stock {
    color: var(--color-text-muted);
    min-width: 90px;
}

.result-precio {
    font-weight: 600;
    min-width: 80px;
    text-align: right;
}

.cart-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
}

.cart-header h2 {
    font-size: 1.4rem;
    margin: 0;
}

.stock-warning-row td {
    background: rgba(229, 62, 62, 0.06);
}

.stock-warning {
    display: block;
    color: #e53e3e;
    font-size: 0.8rem;
}

.btn-primary {
    background: #3F2281;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    cursor: pointer;
}

.btn-primary:hover:not(:disabled) {
    background: #3F2281;
}

.btn-primary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

.btn-secondary {
    background: var(--color-surface-2);
    color: var(--color-text);
    border: none;
    padding: 0.75rem 1rem;
    border-radius: 6px;
    cursor: pointer;
}

.btn-secondary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

.btn-tertiary {
    background: transparent;
    color: var(--color-text);
    border: 1px solid #e53e3e;
    padding: 0.75rem 1rem;
    border-radius: 6px;
    cursor: pointer;
}

.btn-tertiary:disabled {
    opacity: 0.6;
    border:none;
    cursor: not-allowed;
}

.btn-volver {
    padding: 0.5rem 1rem;
}

.cart-table {
    width: 100%;
    border-radius: 12px;
    overflow: hidden;
}

.cart-table th,
.cart-table td {
    padding: 1rem;
    text-align: left;
}

.cart-table th {
    background: var(--color-surface-2);
    font-weight: 600;
}

.cart-input {
    width: 90px;
    padding: 0.5rem;
    border: 1px solid var(--color-border);
    border-radius: 6px;
    background: var(--color-surface);
    color: var(--color-text);
}

.btn-icon {
    background: none;
    border: none;
    cursor: pointer;
    padding: 0.25rem;
}

.btn-icon img {
    width: 18px;
    height: 18px;
}

.btn-danger:hover {
    opacity: 0.7;
}

.form-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
}

.form-group input {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid var(--color-border);
    border-radius: 6px;
    box-sizing: border-box;
    background: var(--color-surface);
    color: var(--color-text);
}

.tipo-select {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid var(--color-border);
    border-radius: 6px;
    box-sizing: border-box;
    background: var(--color-surface);
    color: var(--color-text);
    background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='12' height='8' viewBox='0 0 12 8'><path d='M1 1l5 5 5-5' fill='none' stroke='%236b7280' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'/></svg>");
    background-repeat: no-repeat;
    background-position: right 0.75rem center;
    padding-right: 2.5rem;
}

.error-banner {
    color: #e53e3e;
    background: rgba(229, 62, 62, 0.1);
    border: 1px solid rgba(229, 62, 62, 0.3);
    padding: 0.75rem 1rem;
    border-radius: 6px;
    margin-bottom: 1rem;
}

.cliente-group {
    position: relative;
    min-width: 280px;
}

.cliente-selector {
    display: flex;
    gap: 0.5rem;
}

.cliente-selector input {
    flex: 1;
}

.btn-nuevo-cliente {
    background: var(--color-surface-2);
    color: var(--color-text);
    border: none;
    padding: 0.75rem 1rem;
    border-radius: 6px;
    cursor: pointer;
    white-space: nowrap;
    font-weight: 500;
}

.btn-nuevo-cliente:hover {
    background: var(--color-border);
}

.cliente-dropdown {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    z-index: 30;
    max-height: 220px;
    overflow-y: auto;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    margin-top: 0.25rem;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.cliente-option {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 0.5rem;
    width: 100%;
    text-align: left;
    padding: 0.6rem 0.75rem;
    background: none;
    border: none;
    cursor: pointer;
    color: var(--color-text);
}

.cliente-option:hover {
    background: var(--color-surface-2);
}

.cliente-nombre {
    font-weight: 500;
}

.cliente-tel {
    color: var(--color-text-muted);
    font-size: 0.85rem;
}

.cliente-empty {
    padding: 0.6rem 0.75rem;
    color: var(--color-text-muted);
    font-size: 0.9rem;
}

.cliente-seleccionado {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 0.5rem;
    margin-top: 0.5rem;
    padding: 0.5rem 0.75rem;
    background: var(--color-surface-2);
    border-radius: 6px;
    font-weight: 500;
}

.cliente-limpiar {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--color-text-muted);
    font-size: 1.1rem;
    line-height: 1;
    padding: 0 0.25rem;
}

.cliente-limpiar:hover {
    color: #e53e3e;
}

.modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
    padding: 1rem;
}

.modal-card {
    background: var(--color-surface);
    border-radius: 12px;
    padding: 1.5rem;
    width: 100%;
    max-width: 420px;
    max-height: 90vh;
    overflow-y: auto;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
}

.modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
}

.modal-header h2 {
    margin: 0;
}

.modal-header .btn-icon {
    font-size: 1.25rem;
    color: var(--color-text-muted);
}

.modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    margin-top: 1rem;
}

.error-text {
    color: #e53e3e;
    margin-top: 0.5rem;
    font-size: 0.9rem;
}

.loading,
.empty-state {
    text-align: center;
    padding: 2rem;
    color: var(--color-text-muted);
}

.empty-state.small {
    padding: 1rem;
}
</style>
