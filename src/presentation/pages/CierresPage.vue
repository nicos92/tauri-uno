<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useCierresStore } from "../stores";
import { usePermissions } from "../composables/usePermissions";
import { useToasts } from "../composables/useToasts";
import { useConfirm } from "../composables/useConfirm";
import { formatMoney } from "../utils/format";
import type { CierreWithTipos } from "../../domain/entities";

const cierresStore = useCierresStore();
const { canCreateCierre, canReabrirCierre } = usePermissions();
const { error: toastError, success: toastSuccess } = useToasts();
const { confirm } = useConfirm();

function todayLocal(): string {
    const now = new Date();
    const y = now.getFullYear();
    const m = String(now.getMonth() + 1).padStart(2, "0");
    const d = String(now.getDate()).padStart(2, "0");
    return `${y}-${m}-${d}`;
}

const fecha = ref(todayLocal());
const expanded = ref<Set<number>>(new Set());

onMounted(async () => {
    await cierresStore.fetchCierres();
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

        <div v-if="canCreateCierre()" class="cierre-bar">
            <input v-model="fecha" type="date" class="date-input" />
            <button @click="handleCerrarDia" class="btn-primary">
                Cerrar día
            </button>
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
    margin-bottom: 1.5rem;
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

.btn-secondary {
    background: var(--color-surface-2);
    color: var(--color-text);
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    cursor: pointer;
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
