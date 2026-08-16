<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useStockStore, useArticulosStore } from "../stores";
import { usePermissions } from "../composables/usePermissions";
import { useToasts } from "../composables/useToasts";
import PageHeader from "../components/ui/PageHeader.vue";
import SearchBar from "../components/ui/SearchBar.vue";
import DataTable from "../components/ui/DataTable.vue";
import EntityFormModal from "../components/ui/EntityFormModal.vue";
import ConfirmButton from "../components/ui/ConfirmButton.vue";
import type {
    Stock,
    CreateStockRequest,
    UpdateStockRequest,
} from "../../domain/entities";
import { calcularPrecioVenta } from "../../domain/entities";

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

const loading = computed(
    () => stockStore.loading || articulosStore.loading,
);

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
    return calcularPrecioVenta(editCosto.value, editGanancia.value);
});

const stockCompletos = computed(() => {
    return stockStore.stocks.map((s) => {
        const articulo = articulosStore.articulos.find(
            (a) => a.id === s.id_articulo,
        );
        const precioVenta =
            preciosVenta.value.get(s.id) ||
            calcularPrecioVenta(s.costo, s.ganancia);
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
    const success = await stockStore.deleteStock(id);
    if (!success) {
        useToasts().error(
            stockStore.error || "No se pudo eliminar el stock.",
        );
    }
}
</script>

<template>
    <div class="stock-page">
        <PageHeader title="Gestión de Stock">
            <button
                v-if="canCreateStock()"
                @click="openCreateModal"
                class="btn-primary"
            >
                Crear Stock
            </button>
        </PageHeader>

        <SearchBar
            v-model="searchQuery"
            placeholder="Buscar por código o artículo..."
        />

        <div v-if="stockStore.error" class="error-banner">
            {{ stockStore.error }}
        </div>

        <DataTable
            :columns="['Código', 'Artículo', 'Cantidad', 'Costo', 'Ganancia %', 'Precio Venta', 'Acciones']"
            :loading="loading"
            :count="filteredStock.length"
            empty="No hay stock que coincida con la búsqueda"
        >
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
                    <ConfirmButton
                        v-if="canDeleteStock()"
                        message="¿Está seguro de eliminar este stock?"
                        @confirmed="handleDelete(stock.id)"
                    />
                </td>
            </tr>
        </DataTable>

        <EntityFormModal
            v-model="showCreateModal"
            title="Crear Stock"
            :error="stockStore.error"
            submit-label="Crear"
            :disable-submit="!newIdArticulo"
            @submit="handleCreate"
        >
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
        </EntityFormModal>

        <EntityFormModal
            v-model="showEditModal"
            title="Editar Stock"
            :error="stockStore.error"
            submit-label="Guardar"
            @submit="handleUpdate"
        >
            <div class="form-group">
                <label>Cantidad</label>
                <input
                    v-model.number="editCantidad"
                    type="number"
                    step="0.01"
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
            <template #extra>
                <div class="preview-precio">
                    Precio de Venta: ${{
                        editPreviewPrecioVenta.toFixed(2)
                    }}
                </div>
            </template>
        </EntityFormModal>
    </div>
</template>

<style scoped>
.stock-page {
    padding: 2rem;
    background: var(--color-bg);
    min-height: 100%;
}

.preview-precio {
    background: var(--color-surface-2);
    padding: 0.75rem;
    border-radius: 6px;
    text-align: center;
    font-weight: 500;
    color: var(--color-primary);
    margin-bottom: 1rem;
}
</style>
