<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useCierresStore } from "../stores";
import { usePermissions } from "../composables/usePermissions";
import { useToasts } from "../composables/useToasts";
import { useConfirm } from "../composables/useConfirm";
import { formatMoney } from "../utils/format";
import { todayLocal } from "../utils/date";
import type { CierreWithTipos } from "../../domain/entities";

const cierresStore = useCierresStore();
const { canCreateCierre, canReabrirCierre } = usePermissions();
const { error: toastError, success: toastSuccess } = useToasts();
const { confirm } = useConfirm();

const fecha = ref(todayLocal());
const expanded = ref<Set<number>>(new Set());

const PAGE_SIZE_OPTIONS = [5, 10, 15, 20, 25];

const pageSize = ref(10);
const offset = ref(0);

const currentPage = computed(() => Math.floor(offset.value / pageSize.value) + 1);
const totalPages = computed(() =>
    Math.max(1, Math.ceil(cierresStore.total / pageSize.value))
);

const pages = computed<Array<number | "...">>(() => {
    const total = totalPages.value;
    const current = currentPage.value;
    const window = 2;
    const result: Array<number | "..."> = [];
    for (let p = 1; p <= total; p++) {
        if (
            p === 1 ||
            p === total ||
            (p >= current - window && p <= current + window)
        ) {
            result.push(p);
        } else if (result[result.length - 1] !== "...") {
            result.push("...");
        }
    }
    return result;
});

async function fetchCierres() {
    await cierresStore.fetchCierres({
        limit: pageSize.value,
        offset: offset.value,
    });
}

async function handlePageSizeChange() {
    offset.value = 0;
    await fetchCierres();
}

function goToPage(page: number | "...") {
    if (typeof page !== "number") return;
    if (page === currentPage.value || page < 1 || page > totalPages.value) return;
    offset.value = (page - 1) * pageSize.value;
    fetchCierres();
}

async function nextPage() {
    goToPage(currentPage.value + 1);
}

async function prevPage() {
    goToPage(currentPage.value - 1);
}

onMounted(async () => {
    await fetchCierres();
});

function toggleTipos(id: number) {
    const next = new Set(expanded.value);
    if (next.has(id)) {
        next.delete(id);
    } else {
        next.add(id);
    }
    expanded.value = next;
}

async function handleCerrarDia() {
    if (!fecha.value) {
        toastError("Debe seleccionar una fecha.");
        return;
    }
    if (
        !(await confirm({
            message: `¿Está seguro de cerrar el día ${fecha.value}? Se guardará un snapshot de las ventas del día.`,
            confirmText: "Cerrar día",
        }))
    ) {
        return;
    }
    const ok = await cierresStore.crearCierre(fecha.value);
    if (ok) {
        toastSuccess(`Cierre del día ${fecha.value} generado.`);
        offset.value = 0;
        await fetchCierres();
    } else {
        toastError(cierresStore.error || "No se pudo generar el cierre.");
    }
}

async function handleReabrir(cierre: CierreWithTipos) {
    if (
        !(await confirm({
            message: `¿Está seguro de reabrir el día ${cierre.fecha}? El cierre se eliminará y se podrán registrar nuevas ventas.`,
            confirmText: "Reabrir",
        }))
    ) {
        return;
    }
    const ok = await cierresStore.reabrirCierre(cierre.fecha);
    if (ok) {
        toastSuccess(`Día ${cierre.fecha} reabierto.`);
        offset.value = 0;
        await fetchCierres();
    } else {
        toastError(cierresStore.error || "No se pudo reabrir el día.");
    }
}
</script>

<template>
    <div class="cierres-page">
        <div class="page-header">
            <h1>Cierres del día</h1>
        </div>

        <div class="cierre-bar">
            <input
                v-if="canCreateCierre()"
                v-model="fecha"
                type="date"
                class="date-input"
            />
            <button
                v-if="canCreateCierre()"
                @click="handleCerrarDia"
                class="btn-primary"
            >
                Cerrar día
            </button>
            <div class="toolbar-spacer"></div>
            <select
                v-model="pageSize"
                class="filter-input page-size-select"
                @change="handlePageSizeChange"
                title="Cierres por página"
            >
                <option v-for="n in PAGE_SIZE_OPTIONS" :key="n" :value="n">
                    {{ n }} por página
                </option>
            </select>
        </div>

        <div v-if="cierresStore.loading" class="loading">Cargando...</div>

        <div v-if="cierresStore.error" class="error-banner">
            {{ cierresStore.error }}
        </div>

        <div class="table-wrapper">
        <table v-if="!cierresStore.loading" class="cierres-table">
            <thead>
                <tr>
                    <th>Fecha</th>
                    <th>Día / Mes / Año</th>
                    <th>Total Venta</th>
                    <th>Total Costo</th>
                    <th>Ganancia</th>
                    <th>Desglose</th>
                    <th>Acciones</th>
                </tr>
            </thead>
            <tbody>
                <template v-for="cierre in cierresStore.cierres" :key="cierre.id">
                    <tr>
                        <td>{{ cierre.fecha }}</td>
                        <td>{{ cierre.dia }} / {{ cierre.mes }} / {{ cierre.anio }}</td>
                        <td>{{ formatMoney(cierre.total_venta) }}</td>
                        <td>{{ formatMoney(cierre.total_costo) }}</td>
                        <td>{{ formatMoney(cierre.total_ganancia) }}</td>
                        <td>
                            <button
                                @click="toggleTipos(cierre.id)"
                                class="btn-secondary btn-small"
                            >
                                {{ expanded.has(cierre.id) ? "Ocultar" : "Ver" }}
                            </button>
                        </td>
                        <td>
                            <button
                                v-if="canReabrirCierre()"
                                @click="handleReabrir(cierre)"
                                class="btn-secondary btn-small"
                            >
                                Reabrir
                            </button>
                        </td>
                    </tr>
                    <tr v-if="expanded.has(cierre.id)">
                        <td colspan="7" class="tipos-cell">
                            <div v-if="cierre.tipos.length === 0" class="empty-tipos">
                                Sin ventas en este día.
                            </div>
                            <table v-else class="tipos-table">
                                <thead>
                                    <tr>
                                        <th>Tipo de venta</th>
                                        <th>Total</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr
                                        v-for="tipo in cierre.tipos"
                                        :key="tipo.id_tipo_venta"
                                    >
                                        <td>{{ tipo.tipo_venta }}</td>
                                        <td>{{ formatMoney(tipo.total) }}</td>
                                    </tr>
                                </tbody>
                            </table>
                        </td>
                    </tr>
                </template>
            </tbody>
        </table>
        </div>

        <div v-if="!cierresStore.loading && cierresStore.cierres.length === 0" class="empty-state">
            No hay cierres registrados.
        </div>

        <div v-if="!cierresStore.loading && cierresStore.total > 0" class="pagination">
            <span class="pagination-info">
                Mostrando {{ cierresStore.cierres.length }} de {{ cierresStore.total }} cierres
            </span>
            <div class="pagination-buttons">
                <button
                    @click="prevPage"
                    :disabled="currentPage <= 1"
                    class="btn-secondary"
                >
                    ‹ Anterior
                </button>
                <button
                    v-for="p in pages"
                    :key="p"
                    :class="['page-btn', { active: p === currentPage }]"
                    :disabled="p === '...'"
                    @click="goToPage(p)"
                >
                    {{ p }}
                </button>
                <button
                    @click="nextPage"
                    :disabled="currentPage >= totalPages"
                    class="btn-secondary"
                >
                    Siguiente ›
                </button>
            </div>
        </div>
    </div>
</template>

<style scoped>
.cierres-page {
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

.cierre-bar {
    display: flex;
    gap: 1rem;
    align-items: center;
    flex-wrap: wrap;
    margin-bottom: 1.5rem;
}

.toolbar-spacer {
    flex: 1;
}

.filter-input {
    padding: 0.6rem 0.75rem;
    border: 1px solid var(--color-border);
    border-radius: 6px;
    background: var(--color-surface);
    color: var(--color-text);
    font-size: 0.95rem;
}

select.filter-input {
    background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='12' height='8' viewBox='0 0 12 8'><path d='M1 1l5 5 5-5' fill='none' stroke='%236b7280' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'/></svg>");
    background-repeat: no-repeat;
    background-position: right 0.75rem center;
    padding-right: 2.5rem;
}

.date-input {
    padding: 0.75rem 1rem;
    border: 1px solid var(--color-border);
    border-radius: 6px;
    font-size: 1rem;
    box-sizing: border-box;
    background: var(--color-surface);
    color: var(--color-text);
}

.date-input:focus {
    outline: none;
    border-color: var(--color-primary);
}

.btn-primary {
    background: var(--color-primary);
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    cursor: pointer;
}

.btn-primary:hover:not(:disabled) {
    background: var(--color-secondary);
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
    opacity: 0.5;
    cursor: not-allowed;
}

.btn-small {
    padding: 0.4rem 0.75rem;
    font-size: 0.85rem;
}

.table-wrapper {
    overflow-x: auto;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.cierres-table,
.tipos-table {
    width: 100%;
    background: var(--color-surface);
}

.cierres-table th,
.cierres-table td,
.tipos-table th,
.tipos-table td {
    padding: 1rem;
    text-align: left;
}

.cierres-table th,
.tipos-table th {
    background: var(--color-surface-2);
    font-weight: 600;
}

.tipos-cell {
    background: var(--color-surface);
}

.tipos-table {
    border-radius: 12px;
    overflow: hidden;
    box-shadow: none;
}

.tipos-table th {
    background: var(--color-surface-2);
}

.empty-tipos {
    color: var(--color-text-muted);
}

.error-banner {
    color: var(--color-danger);
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

.pagination {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
    margin-top: 1.5rem;
}

.pagination-info {
    color: var(--color-text-muted);
    font-size: 0.9rem;
    white-space: nowrap;
    text-align: center;
}

.pagination-buttons {
    display: flex;
    gap: 0.4rem;
    align-items: center;
    justify-content: center;
    flex-wrap: wrap;
}

.pagination-buttons .btn-secondary {
    padding: 0.5rem 1rem;
}

.page-btn {
    min-width: 2.2rem;
    height: 2.2rem;
    padding: 0 0.5rem;
    border: 1px solid var(--color-border);
    border-radius: 6px;
    background: var(--color-surface);
    color: var(--color-text);
    font-size: 0.9rem;
    cursor: pointer;
}

.page-btn:disabled {
    border: none;
    background: none;
    cursor: default;
}

.page-btn.active {
    background: var(--color-primary);
    border-color: var(--color-primary);
    color: #fff;
    font-weight: 600;
}
</style>
