<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useStockStore, useArticulosStore } from "../stores";
import { usePermissions } from "../composables/usePermissions";
import type {
    Stock,
    CreateStockRequest,
    UpdateStockRequest,
} from "../../domain/entities";

const stockStore = useStockStore();
const articulosStore = useArticulosStore();
const { canCreateStock, canUpdateStock, canDeleteStock } = usePermissions();

const showCreateModal = ref(false);
const showEditModal = ref(false);
const selectedStock = ref<Stock | null>(null);

const newIdArticulo = ref<number | null>(null);
const newCantidad = ref(0);
const newCosto = ref(0);
const newGanancia = ref(0);

const editCantidad = ref(0);
const editCosto = ref(0);
const editGanancia = ref(0);

const preciosVenta = ref<Map<number, number>>(new Map());

const searchQuery = ref("");

const filteredStock = computed(() => {
    const query = searchQuery.value.toLowerCase().trim();
    if (!query) return stockCompletos.value;
    return stockCompletos.value.filter(
        (s) =>
            s.codArticulo.toLowerCase().includes(query) ||
            s.articuloNombre.toLowerCase().includes(query),
    );
});

const editPreviewPrecioVenta = computed(() => {
    return stockStore.calcularPrecioVenta(editCosto.value, editGanancia.value);
});

const stockCompletos = computed(() => {
    return stockStore.stocks.map((s) => {
        const articulo = articulosStore.articulos.find(
            (a) => a.id === s.id_articulo,
        );
        const precioVenta =
            preciosVenta.value.get(s.id) ||
            stockStore.calcularPrecioVenta(s.costo, s.ganancia);
        return {
            ...s,
            articuloNombre: articulo?.articulo || "Sin artículo",
            codArticulo: articulo?.cod_articulo || "-",
            precioVenta,
        };
    });
});

const articulosDisponibles = computed(() => {
    return articulosStore.articulos.filter((a) => {
        const tieneStock = stockStore.stocks.some(
            (s) => s.id_articulo === a.id,
        );
        return !tieneStock;
    });
});

onMounted(async () => {
    await Promise.all([
        stockStore.fetchStock(),
        articulosStore.fetchArticulos(),
    ]);
    for (const stock of stockStore.stocks) {
        const precio = await stockStore.getPrecioVenta(stock.id);
        if (precio !== null) {
            preciosVenta.value.set(stock.id, precio);
        }
    }
});

function openCreateModal() {
    newIdArticulo.value = null;
    newCantidad.value = 0;
    newCosto.value = 0;
    newGanancia.value = 0;
    showCreateModal.value = true;
}

function openEditModal(stock: (typeof stockCompletos.value)[0]) {
    selectedStock.value = stock;
    editCantidad.value = stock.cantidad;
    editCosto.value = stock.costo;
    editGanancia.value = stock.ganancia;
    showEditModal.value = true;
}

async function handleCreate() {
    if (!newIdArticulo.value) return;
    const request: CreateStockRequest = {
        id_articulo: newIdArticulo.value,
        cantidad: newCantidad.value,
        costo: newCosto.value,
        ganancia: newGanancia.value,
    };
    const success = await stockStore.createStock(request);
    if (success) {
        showCreateModal.value = false;
        await articulosStore.fetchArticulos();
    }
}

async function handleUpdate() {
    if (!selectedStock.value) return;
    const request: UpdateStockRequest = {
        id: selectedStock.value.id,
        cantidad: editCantidad.value,
        costo: editCosto.value,
        ganancia: editGanancia.value,
    };
    const success = await stockStore.updateStock(request);
    if (success) {
        preciosVenta.value.set(
            selectedStock.value.id,
            editPreviewPrecioVenta.value,
        );
        showEditModal.value = false;
    }
}

async function handleDelete(id: number) {
    if (confirm("¿Está seguro de eliminar este stock?")) {
        await stockStore.deleteStock(id);
    }
}
</script>

<template>
    <div class="stock-page">
        <div class="page-header">
            <h1>Gestión de Stock</h1>
            <button
                v-if="canCreateStock()"
                @click="openCreateModal"
                class="btn-primary"
            >
                Crear Stock
            </button>
        </div>

        <div class="search-bar">
            <input
                v-model="searchQuery"
                type="text"
                placeholder="Buscar por código o artículo..."
                class="search-input"
            />
        </div>

        <div
            v-if="stockStore.loading || articulosStore.loading"
            class="loading"
        >
            Cargando...
        </div>

        <div v-else-if="stockStore.error" class="error-message">
            {{ stockStore.error }}
        </div>

        <table v-else class="stock-table">
            <thead>
                <tr>
                    <th>Código</th>
                    <th>Artículo</th>
                    <th>Cantidad</th>
                    <th>Costo</th>
                    <th>Ganancia %</th>
                    <th>Precio Venta</th>
                    <th>Acciones</th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="stock in filteredStock" :key="stock.id">
                    <td>{{ stock.codArticulo }}</td>
                    <td>{{ stock.articuloNombre }}</td>
                    <td>{{ stock.cantidad }}</td>
                    <td>${{ stock.costo.toFixed(2) }}</td>
                    <td>{{ stock.ganancia }}%</td>
                    <td>${{ stock.precioVenta.toFixed(2) }}</td>
                    <td class="actions">
                        <button
                            v-if="canUpdateStock()"
                            @click="openEditModal(stock)"
                            class="btn-icon"
                            title="Editar"
                        >
                            <img src="/svg/edit.svg" alt="Editar" />
                        </button>
                        <button
                            v-if="canDeleteStock()"
                            @click="handleDelete(stock.id)"
                            class="btn-icon btn-danger"
                            title="Eliminar"
                        >
                            <img src="/svg/trash.svg" alt="Eliminar" />
                        </button>
                    </td>
                </tr>
            </tbody>
        </table>

        <div v-if="filteredStock.length === 0" class="empty-state">
            No hay stock que coincida con la búsqueda
        </div>

        <div
            v-if="showCreateModal"
            class="modal-overlay"
            @click.self="showCreateModal = false"
        >
            <div class="modal">
                <h2>Crear Stock</h2>
                <form @submit.prevent="handleCreate">
                    <div class="form-group">
                        <label>Artículo</label>
                        <select v-model="newIdArticulo" required>
                            <option :value="null" disabled>
                                Seleccione un artículo
                            </option>
                            <option
                                v-for="art in articulosDisponibles"
                                :key="art.id"
                                :value="art.id"
                            >
                                {{ art.cod_articulo }} - {{ art.articulo }}
                            </option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label>Cantidad</label>
                        <input
                            v-model.number="newCantidad"
                            type="number"
                            step="0.01"
                            min="0"
                            required
                        />
                    </div>
                    <div class="form-group">
                        <label>Costo</label>
                        <input
                            v-model.number="newCosto"
                            type="number"
                            step="0.01"
                            min="0"
                            required
                        />
                    </div>
                    <div class="form-group">
                        <label>Ganancia (%)</label>
                        <input
                            v-model.number="newGanancia"
                            type="number"
                            step="0.01"
                            min="0"
                            required
                        />
                    </div>
                    <div v-if="stockStore.error" class="error-message">
                        {{ stockStore.error }}
                    </div>
                    <div class="modal-actions">
                        <button
                            type="button"
                            @click="showCreateModal = false"
                            class="btn-secondary"
                        >
                            Cancelar
                        </button>
                        <button
                            type="submit"
                            class="btn-primary"
                            :disabled="!newIdArticulo"
                        >
                            Crear
                        </button>
                    </div>
                </form>
            </div>
        </div>

        <div
            v-if="showEditModal"
            class="modal-overlay"
            @click.self="showEditModal = false"
        >
            <div class="modal">
                <h2>Editar Stock</h2>
                <form @submit.prevent="handleUpdate">
                    <div class="form-group">
                        <label>Cantidad</label>
                        <input
                            v-model.number="editCantidad"
                            type="number"
                            step="0.01"
                            min="0"
                            required
                        />
                    </div>
                    <div class="form-group">
                        <label>Costo</label>
                        <input
                            v-model.number="editCosto"
                            type="number"
                            step="0.01"
                            min="0"
                            required
                        />
                    </div>
                    <div class="form-group">
                        <label>Ganancia (%)</label>
                        <input
                            v-model.number="editGanancia"
                            type="number"
                            step="0.01"
                            min="0"
                            required
                        />
                    </div>
                    <div class="preview-precio">
                        Precio de Venta: ${{
                            editPreviewPrecioVenta.toFixed(2)
                        }}
                    </div>
                    <div v-if="stockStore.error" class="error-message">
                        {{ stockStore.error }}
                    </div>
                    <div class="modal-actions">
                        <button
                            type="button"
                            @click="showEditModal = false"
                            class="btn-secondary"
                        >
                            Cancelar
                        </button>
                        <button type="submit" class="btn-primary">
                            Guardar
                        </button>
                    </div>
                </form>
            </div>
        </div>
    </div>
</template>

<style scoped>
.stock-page {
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

.stock-table {
    width: 100%;
    background: var(--color-surface);
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.stock-table th,
.stock-table td {
    padding: 1rem;
    text-align: left;
}

.stock-table th {
    background: var(--color-surface-2);
    font-weight: 600;
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

.modal-actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
    margin-top: 1.5rem;
}

.preview-precio {
    background: var(--color-surface-2);
    padding: 0.75rem;
    border-radius: 6px;
    text-align: center;
    font-weight: 500;
    color: #667eea;
    margin-bottom: 1rem;
}

.error-message {
    color: #e53e3e;
    margin-bottom: 1rem;
}

.loading,
.empty-state {
    text-align: center;
    padding: 2rem;
    color: var(--color-text-muted);
}
</style>
