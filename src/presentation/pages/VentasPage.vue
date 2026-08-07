<script setup lang="ts">
import { ref, computed, onMounted, nextTick } from "vue";
import { useVentasStore, useStockStore, useArticulosStore } from "../stores";
import { usePermissions } from "../composables/usePermissions";
import { useToasts } from "../composables/useToasts";
import type { CreateVentaRequest, VentaWithDetalle } from "../../domain/entities";

const ventasStore = useVentasStore();
const stockStore = useStockStore();
const articulosStore = useArticulosStore();
const {
    canCreateVenta,
    canAnularVenta,
    canGenerarPresupuesto,
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

const showCreateModal = ref(false);
const showDetailModal = ref(false);
const selectedVenta = ref<VentaWithDetalle | null>(null);

const selectedArticuloId = ref<number | null>(null);
const observacion = ref("");
const cart = ref<CartItem[]>([]);

const searchQuery = ref("");

const filteredVentas = computed(() => {
    const query = searchQuery.value.toLowerCase().trim();
    if (!query) return ventasStore.ventas;
    return ventasStore.ventas.filter(
        (v) =>
            v.username.toLowerCase().includes(query) ||
            v.items.some(
                (i) =>
                    i.articulo.toLowerCase().includes(query) ||
                    i.cod_articulo.toLowerCase().includes(query),
            ),
    );
});

const articulosVendibles = computed(() => {
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

const articulosParaAgregar = computed(() => {
    const inCart = new Set(cart.value.map((c) => c.id_articulo));
    return articulosVendibles.value.filter((a) => !inCart.has(a.id_articulo));
});

const carritoTotal = computed(() =>
    cart.value.reduce((acc, item) => acc + item.subtotal, 0),
);

const carritoValido = computed(
    () => cart.value.length > 0 && cart.value.every((i) => i.cantidad > 0),
);

const fechaHoy = computed(() => new Date().toLocaleDateString());

onMounted(async () => {
    await Promise.all([
        ventasStore.fetchVentas(),
        stockStore.fetchStock(),
        articulosStore.fetchArticulos(),
    ]);
});

function openCreateModal() {
    cart.value = [];
    observacion.value = "";
    selectedArticuloId.value = null;
    showCreateModal.value = true;
}

function addArticulo() {
    if (!selectedArticuloId.value) return;
    const articulo = articulosVendibles.value.find(
        (a) => a.id_articulo === selectedArticuloId.value,
    );
    if (!articulo) return;
    cart.value.push({
        id_articulo: articulo.id_articulo,
        cod_articulo: articulo.cod_articulo,
        articulo: articulo.articulo,
        stockDisponible: articulo.stockDisponible,
        cantidad: 1,
        precio: articulo.precioVenta,
        subtotal: articulo.precioVenta,
    });
    selectedArticuloId.value = null;
}

function removeArticulo(idArticulo: number) {
    cart.value = cart.value.filter((c) => c.id_articulo !== idArticulo);
}

function updateSubtotal(item: CartItem) {
    item.subtotal = item.cantidad * item.precio;
}

function formatMoney(value: number): string {
    return `$${value.toFixed(2)}`;
}

async function handleCreate() {
    if (!carritoValido.value) return;
    const request: CreateVentaRequest = {
        items: cart.value.map((item) => ({
            id_articulo: item.id_articulo,
            cantidad: item.cantidad,
            precio_unitario: item.precio,
        })),
        observacion: observacion.value.trim() || undefined,
    };
    const success = await ventasStore.createVenta(request);
    if (success) {
        toastSuccess("Venta registrada correctamente.");
        showCreateModal.value = false;
    } else {
        toastError(ventasStore.error || "No se pudo registrar la venta.");
    }
}

function openDetailModal(venta: VentaWithDetalle) {
    selectedVenta.value = venta;
    showDetailModal.value = true;
}

async function handleAnular(id: number) {
    if (!confirm("¿Está seguro de anular esta venta? Se devolverán las cantidades al stock.")) {
        return;
    }
    const success = await ventasStore.anularVenta(id);
    if (success) {
        toastSuccess("Venta anulada y stock restaurado.");
    } else {
        toastError(ventasStore.error || "No se pudo anular la venta.");
    }
}

async function generarPdf() {
    if (cart.value.length === 0) return;
    await nextTick();
    window.print();
}
</script>

<template>
    <div class="ventas-page">
        <div class="page-header">
            <h1>Gestión de Ventas</h1>
            <button
                v-if="canCreateVenta()"
                @click="openCreateModal"
                class="btn-primary"
            >
                Nueva Venta
            </button>
        </div>

        <div class="search-bar">
            <input
                v-model="searchQuery"
                type="text"
                placeholder="Buscar por usuario o artículo..."
                class="search-input"
            />
        </div>

        <div v-if="ventasStore.loading" class="loading">Cargando...</div>

        <div v-if="ventasStore.error" class="error-banner">
            {{ ventasStore.error }}
        </div>

        <table v-if="!ventasStore.loading" class="ventas-table">
            <thead>
                <tr>
                    <th>N°</th>
                    <th>Fecha</th>
                    <th>Usuario</th>
                    <th>Items</th>
                    <th>Total</th>
                    <th>Estado</th>
                    <th>Acciones</th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="venta in filteredVentas" :key="venta.id">
                    <td>{{ venta.id }}</td>
                    <td>{{ new Date(venta.fecha).toLocaleString() }}</td>
                    <td>{{ venta.username }}</td>
                    <td>{{ venta.items.length }}</td>
                    <td>{{ formatMoney(venta.total) }}</td>
                    <td>
                        <span
                            :class="
                                venta.anulada
                                    ? 'status-anulada'
                                    : 'status-activa'
                            "
                        >
                            {{ venta.anulada ? "Anulada" : "Activa" }}
                        </span>
                    </td>
                    <td class="actions">
                        <button
                            @click="openDetailModal(venta)"
                            class="btn-icon"
                            title="Ver detalle"
                        >
                            <img src="/svg/article.svg" alt="Ver detalle" />
                        </button>
                        <button
                            v-if="canAnularVenta() && !venta.anulada"
                            @click="handleAnular(venta.id)"
                            class="btn-icon btn-danger"
                            title="Anular venta"
                        >
                            <img src="/svg/trash.svg" alt="Anular venta" />
                        </button>
                    </td>
                </tr>
            </tbody>
        </table>

        <div v-if="filteredVentas.length === 0" class="empty-state">
            No hay ventas que coincidan con la búsqueda
        </div>

        <div
            v-if="showCreateModal"
            class="modal-overlay"
            @click.self="showCreateModal = false"
        >
            <div class="modal modal-large">
                <h2>Nueva Venta</h2>

                <div class="form-group">
                    <label>Agregar artículo</label>
                    <div class="add-articulo-row">
                        <select v-model="selectedArticuloId">
                            <option :value="null" disabled>
                                Seleccione un artículo
                            </option>
                            <option
                                v-for="art in articulosParaAgregar"
                                :key="art.id_articulo"
                                :value="art.id_articulo"
                            >
                                {{ art.cod_articulo }} - {{ art.articulo }}
                                (Stock: {{ art.stockDisponible }})
                            </option>
                        </select>
                        <button
                            type="button"
                            @click="addArticulo"
                            class="btn-primary btn-add"
                            :disabled="!selectedArticuloId"
                        >
                            Agregar
                        </button>
                    </div>
                </div>

                <table v-if="cart.length > 0" class="cart-table">
                    <thead>
                        <tr>
                            <th>Código</th>
                            <th>Artículo</th>
                            <th>Cantidad</th>
                            <th>Precio</th>
                            <th>Subtotal</th>
                            <th></th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr v-for="item in cart" :key="item.id_articulo">
                            <td>{{ item.cod_articulo }}</td>
                            <td>{{ item.articulo }}</td>
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

                <div v-if="cart.length > 0" class="cart-total">
                    Total: {{ formatMoney(carritoTotal) }}
                </div>

                <div v-if="cart.length === 0" class="empty-state">
                    Agregue artículos a la venta
                </div>

                <div class="form-group">
                    <label>Observación</label>
                    <input
                        v-model="observacion"
                        type="text"
                        placeholder="Opcional"
                    />
                </div>

                <div v-if="ventasStore.error" class="error-message">
                    {{ ventasStore.error }}
                </div>

                <div class="modal-actions">
                    <button
                        v-if="canGenerarPresupuesto()"
                        type="button"
                        @click="generarPdf"
                        class="btn-secondary"
                        :disabled="cart.length === 0"
                    >
                        Generar PDF
                    </button>
                    <button
                        type="button"
                        @click="showCreateModal = false"
                        class="btn-secondary"
                    >
                        Cancelar
                    </button>
                    <button
                        type="button"
                        @click="handleCreate"
                        class="btn-primary"
                        :disabled="!carritoValido"
                    >
                        Registrar Venta
                    </button>
                </div>
            </div>
        </div>

        <div
            v-if="showDetailModal && selectedVenta"
            class="modal-overlay"
            @click.self="showDetailModal = false"
        >
            <div class="modal modal-large">
                <h2>Venta N° {{ selectedVenta.id }}</h2>
                <p class="detail-meta">
                    Fecha: {{ new Date(selectedVenta.fecha).toLocaleString() }}
                    — Usuario: {{ selectedVenta.username }}
                    <span
                        :class="
                            selectedVenta.anulada
                                ? 'status-anulada'
                                : 'status-activa'
                        "
                    >
                        {{ selectedVenta.anulada ? "Anulada" : "Activa" }}
                    </span>
                </p>
                <p v-if="selectedVenta.observacion" class="detail-meta">
                    Observación: {{ selectedVenta.observacion }}
                </p>
                <table class="cart-table">
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
                        <tr
                            v-for="item in selectedVenta.items"
                            :key="item.id"
                        >
                            <td>{{ item.cod_articulo }}</td>
                            <td>{{ item.articulo }}</td>
                            <td>{{ item.cantidad }}</td>
                            <td>{{ formatMoney(item.precio_unitario) }}</td>
                            <td>{{ formatMoney(item.subtotal) }}</td>
                        </tr>
                    </tbody>
                </table>
                <div class="cart-total">
                    Total: {{ formatMoney(selectedVenta.total) }}
                </div>
                <div class="modal-actions">
                    <button
                        @click="showDetailModal = false"
                        class="btn-secondary"
                    >
                        Cerrar
                    </button>
                </div>
            </div>
        </div>

        <div class="print-area" id="print-area">
            <h1>Presupuesto</h1>
            <p>Fecha: {{ fechaHoy }}</p>
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
            <p class="print-total">Total: {{ formatMoney(carritoTotal) }}</p>
            <p v-if="observacion" class="print-obs">
                Observación: {{ observacion }}
            </p>
        </div>
    </div>
</template>

<style scoped>
.ventas-page {
    padding: 2rem;
    background: var(--color-bg);
    min-height: 100%;
}

.page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
}

.page-header h1 {
    margin: 0;
}

.search-bar {
    margin-bottom: 1.5rem;
}

.search-input {
    width: 100%;
    padding: 0.75rem 1rem;
    border: 1px solid var(--color-border);
    border-radius: 6px;
    font-size: 1rem;
    box-sizing: border-box;
    background: var(--color-surface);
    color: var(--color-text);
}

.search-input:focus {
    outline: none;
    border-color: #667eea;
}

.btn-primary {
    background: #667eea;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    cursor: pointer;
}

.btn-primary:hover:not(:disabled) {
    background: #5568d3;
}

.btn-primary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

.btn-secondary {
    background: var(--color-surface-2);
    color: var(--color-text);
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    cursor: pointer;
}

.btn-secondary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

.ventas-table,
.cart-table {
    width: 100%;
    background: var(--color-surface);
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.ventas-table th,
.ventas-table td,
.cart-table th,
.cart-table td {
    padding: 1rem;
    text-align: left;
}

.ventas-table th,
.cart-table th {
    background: var(--color-surface-2);
    font-weight: 600;
}

.status-activa {
    color: #38a169;
    font-weight: 500;
}

.status-anulada {
    color: #e53e3e;
    font-weight: 500;
}

.actions {
    display: flex;
    gap: 0.5rem;
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

.modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
}

.modal {
    background: var(--color-surface);
    padding: 2rem;
    border-radius: 12px;
    width: 100%;
    max-width: 500px;
    max-height: 90vh;
    overflow-y: auto;
}

.modal-large {
    max-width: 720px;
}

.modal h2 {
    margin: 0 0 1.5rem;
}

.form-group {
    margin-bottom: 1rem;
}

.form-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
}

.form-group input,
.form-group select {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid var(--color-border);
    border-radius: 6px;
    box-sizing: border-box;
    background: var(--color-surface);
    color: var(--color-text);
}

.add-articulo-row {
    display: flex;
    gap: 0.5rem;
}

.add-articulo-row select {
    flex: 1;
}

.cart-input {
    width: 90px;
    padding: 0.5rem;
    border: 1px solid var(--color-border);
    border-radius: 6px;
    background: var(--color-surface);
    color: var(--color-text);
}

.cart-total {
    margin-top: 1rem;
    font-size: 1.1rem;
    font-weight: 600;
    text-align: right;
}

.detail-meta {
    margin-bottom: 0.5rem;
    color: var(--color-text-muted);
}

.modal-actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
    margin-top: 1.5rem;
}

.error-message {
    color: #e53e3e;
    margin-bottom: 1rem;
}

.error-banner {
    color: #e53e3e;
    background: rgba(229, 62, 62, 0.1);
    border: 1px solid rgba(229, 62, 62, 0.3);
    padding: 0.75rem 1rem;
    border-radius: 6px;
    margin-bottom: 1rem;
}

.loading,
.empty-state {
    text-align: center;
    padding: 2rem;
    color: var(--color-text-muted);
}

.print-area {
    display: none;
}

@media print {
    body * {
        visibility: hidden;
    }

    .print-area,
    .print-area * {
        visibility: visible;
    }

    .print-area {
        display: block;
        position: absolute;
        left: 0;
        top: 0;
        width: 100%;
        padding: 1rem;
    }

    .print-area table {
        width: 100%;
        border-collapse: collapse;
    }

    .print-area th,
    .print-area td {
        border: 1px solid #000;
        padding: 0.5rem;
        text-align: left;
    }

    .print-total {
        font-weight: bold;
        text-align: right;
    }

    .print-obs {
        margin-top: 1rem;
    }
}
</style>
