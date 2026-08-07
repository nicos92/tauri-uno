<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useAuditStore } from "../stores";
import type { AuditLogFilters } from "../../domain/entities";

const auditStore = useAuditStore();

const PAGE_SIZE = 50;

const screen = ref("");
const action = ref("");
const from = ref("");
const to = ref("");
const offset = ref(0);
const hasMore = ref(false);

const screens = [
    "Usuarios",
    "Proveedores",
    "Categorias",
    "SubCategorias",
    "Articulos",
    "Stock",
    "Ventas",
    "Permisos",
    "Auditoria",
];

const actions = ["nuevo", "modificar", "consultar", "eliminar"];

function buildFilters(): AuditLogFilters {
    const filters: AuditLogFilters = {
        limit: PAGE_SIZE,
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
    hasMore.value = auditStore.logs.length === PAGE_SIZE;
}

async function handleSearch() {
    offset.value = 0;
    await fetchLogs();
}

async function nextPage() {
    offset.value += PAGE_SIZE;
    await fetchLogs();
}

async function prevPage() {
    offset.value = Math.max(0, offset.value - PAGE_SIZE);
    await fetchLogs();
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
            <button @click="handleSearch" class="btn-primary">Buscar</button>
            <button @click="resetFilters" class="btn-secondary">
                Limpiar
            </button>
        </div>

        <div v-if="auditStore.loading" class="loading">Cargando...</div>

        <div v-if="auditStore.error" class="error-banner">
            {{ auditStore.error }}
        </div>

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

        <div
            v-if="!auditStore.loading && auditStore.logs.length === 0"
            class="empty-state"
        >
            No hay registros de auditoría con los filtros seleccionados
        </div>

        <div v-if="!auditStore.loading && auditStore.logs.length > 0" class="pagination">
            <button
                @click="prevPage"
                :disabled="offset === 0"
                class="btn-secondary"
            >
                Anterior
            </button>
            <button @click="nextPage" :disabled="!hasMore" class="btn-secondary">
                Siguiente
            </button>
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

.audit-table {
    width: 100%;
    background: var(--color-surface);
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
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
    gap: 1rem;
    justify-content: flex-end;
    margin-top: 1.5rem;
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
