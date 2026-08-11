<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useAuditStore } from "../stores";
import type { AuditLogFilters } from "../../domain/entities";

const auditStore = useAuditStore();

const PAGE_SIZE_OPTIONS = [5, 10, 15, 20, 25];

const pageSize = ref(10);

const screen = ref("");
const action = ref("");
const from = ref("");
const to = ref("");
const offset = ref(0);

const screens = [
    "Usuarios",
    "Proveedores",
    "Clientes",
    "Categorias",
    "SubCategorias",
    "Articulos",
    "Stock",
    "Ventas",
    "Tipos de Venta",
    "Permisos",
    "Auditoria",
    "Cierres del día",
];

const actions = ["nuevo", "modificar", "consultar", "eliminar"];

const currentPage = computed(() => Math.floor(offset.value / pageSize.value) + 1);
const totalPages = computed(() => Math.max(1, Math.ceil(auditStore.total / pageSize.value)));

const pages = computed<Array<number | "...">>(() => {
    const total = totalPages.value;
    const current = currentPage.value;
    const window = 2;
    const result: Array<number | "..."> = [];
    for (let p = 1; p <= total; p++) {
        if (p === 1 || p === total || (p >= current - window && p <= current + window)) {
            result.push(p);
        } else if (result[result.length - 1] !== "...") {
            result.push("...");
        }
    }
    return result;
});

function buildFilters(): AuditLogFilters {
    const filters: AuditLogFilters = {
        limit: pageSize.value,
        offset: offset.value,
    };
    if (screen.value) filters.screen = screen.value;
    if (action.value) filters.action = action.value;
    if (from.value) filters.from = `${from.value}T00:00:00.000Z`;
    if (to.value) filters.to = `${to.value}T23:59:59.999Z`;
    return filters;
}

async function fetchLogs() {
    await auditStore.fetchLogs(buildFilters());
    if (auditStore.logs.length === 0 && offset.value > 0 && auditStore.total > 0) {
        const lastPage = Math.max(1, Math.ceil(auditStore.total / pageSize.value));
        offset.value = (lastPage - 1) * pageSize.value;
        await auditStore.fetchLogs(buildFilters());
    }
}

async function handleSearch() {
    offset.value = 0;
    await fetchLogs();
}

async function handlePageSizeChange() {
    offset.value = 0;
    await fetchLogs();
}

function goToPage(page: number | "...") {
    if (typeof page !== "number") return;
    if (page === currentPage.value || page < 1 || page > totalPages.value) return;
    offset.value = (page - 1) * pageSize.value;
    fetchLogs();
}

async function nextPage() {
    goToPage(currentPage.value + 1);
}

async function prevPage() {
    goToPage(currentPage.value - 1);
}

function resetFilters() {
    screen.value = "";
    action.value = "";
    from.value = "";
    to.value = "";
    handleSearch();
}

function formatDate(iso: string): string {
    const date = new Date(iso);
    if (isNaN(date.getTime())) return iso;
    return date.toLocaleString();
}

onMounted(() => {
    fetchLogs();
});
</script>

<template>
    <div class="auditoria-page">
        <div class="page-header">
            <h1>Auditoría de Acciones</h1>
        </div>

        <div class="filters-bar">
            <select v-model="screen" class="filter-input">
                <option value="">Todas las pantallas</option>
                <option v-for="s in screens" :key="s" :value="s">
                    {{ s }}
                </option>
            </select>
            <select v-model="action" class="filter-input">
                <option value="">Todas las acciones</option>
                <option v-for="a in actions" :key="a" :value="a">
                    {{ a }}
                </option>
            </select>
            <div class="filter-date">
                <input v-model="from" type="date" class="filter-input" />
                <span>→</span>
                <input v-model="to" type="date" class="filter-input" />
            </div>
            <select
                v-model="pageSize"
                class="filter-input page-size-select"
                @change="handlePageSizeChange"
                title="Registros por página"
            >
                <option v-for="n in PAGE_SIZE_OPTIONS" :key="n" :value="n">
                    {{ n }} por página
                </option>
            </select>
            <button @click="handleSearch" class="btn-primary">Buscar</button>
            <button @click="resetFilters" class="btn-secondary">
                Limpiar
            </button>
        </div>

        <div v-if="auditStore.loading" class="loading">Cargando...</div>

        <div v-if="auditStore.error" class="error-banner">
            {{ auditStore.error }}
        </div>

        <div class="table-wrapper">
        <table v-if="!auditStore.loading" class="audit-table">
            <thead>
                <tr>
                    <th>Fecha y Hora</th>
                    <th>Usuario</th>
                    <th>Pantalla</th>
                    <th>Acción</th>
                    <th>Detalle</th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="log in auditStore.logs" :key="log.id">
                    <td>{{ formatDate(log.created_at) }}</td>
                    <td>{{ log.username || `(id ${log.user_id})` }}</td>
                    <td>{{ log.screen }}</td>
                    <td>
                        <span
                            :class="['badge', `badge-${log.action}`]"
                        >
                            {{ log.action }}
                        </span>
                    </td>
                    <td>{{ log.detail || "-" }}</td>
                </tr>
            </tbody>
        </table>
        </div>

        <div
            v-if="!auditStore.loading && auditStore.logs.length === 0"
            class="empty-state"
        >
            No hay registros de auditoría con los filtros seleccionados
        </div>

        <div v-if="!auditStore.loading && auditStore.total > 0" class="pagination">
            <span class="pagination-info">
                Mostrando {{ auditStore.logs.length }} de {{ auditStore.total }} registros
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
.auditoria-page {
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

.filters-bar {
    display: flex;
    gap: 1rem;
    align-items: center;
    flex-wrap: wrap;
    margin-bottom: 1.5rem;
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

.filter-date {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.btn-primary {
    background: #667eea;
    color: white;
    border: none;
    padding: 0.6rem 1.25rem;
    border-radius: 6px;
    cursor: pointer;
}

.btn-primary:hover:not(:disabled) {
    background: #5568d3;
}

.btn-secondary {
    background: var(--color-surface-2);
    color: var(--color-text);
    border: none;
    padding: 0.6rem 1.25rem;
    border-radius: 6px;
    cursor: pointer;
}

.btn-secondary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.table-wrapper {
    overflow-x: auto;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.audit-table {
    width: 100%;
    background: var(--color-surface);
}

.audit-table th,
.audit-table td {
    padding: 0.9rem 1rem;
    text-align: left;
}

.audit-table th {
    background: var(--color-surface-2);
    font-weight: 600;
}

.badge {
    display: inline-block;
    padding: 0.2rem 0.6rem;
    border-radius: 999px;
    font-size: 0.8rem;
    font-weight: 600;
    text-transform: capitalize;
}

.badge-nuevo {
    background: rgba(72, 187, 120, 0.15);
    color: #276749;
}

.badge-modificar {
    background: rgba(236, 201, 75, 0.2);
    color: #975a16;
}

.badge-consultar {
    background: rgba(66, 153, 225, 0.15);
    color: #2b6cb0;
}

.badge-eliminar {
    background: rgba(229, 62, 62, 0.15);
    color: #c53030;
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
    background: #667eea;
    border-color: #667eea;
    color: #fff;
    font-weight: 600;
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
</style>
