<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useTiposVentaStore } from "../stores";
import { usePermissions } from "../composables/usePermissions";
import { useToasts } from "../composables/useToasts";
import { useConfirm } from "../composables/useConfirm";
import type {
    CreateTipoVentaRequest,
    TipoVenta,
    UpdateTipoVentaRequest,
} from "../../domain/entities";

const tiposVentaStore = useTiposVentaStore();
const { canCreateTipoVenta, canUpdateTipoVenta, canDeleteTipoVenta } =
    usePermissions();
const { confirm } = useConfirm();

const showCreateModal = ref(false);
const showEditModal = ref(false);
const selectedTipo = ref<TipoVenta | null>(null);

const newNombre = ref("");
const newHaciaDonde = ref("");
const editNombre = ref("");
const editHaciaDonde = ref("");

onMounted(async () => {
    await tiposVentaStore.fetchTiposVenta();
});

function openCreateModal() {
    newNombre.value = "";
    newHaciaDonde.value = "";
    showCreateModal.value = true;
}

function openEditModal(tipo: TipoVenta) {
    selectedTipo.value = tipo;
    editNombre.value = tipo.nombre;
    editHaciaDonde.value = tipo.hacia_donde || "";
    showEditModal.value = true;
}

async function handleCreate() {
    const request: CreateTipoVentaRequest = {
        nombre: newNombre.value,
        hacia_donde: newHaciaDonde.value.trim() || undefined,
    };
    const success = await tiposVentaStore.createTipoVenta(request);
    if (success) {
        showCreateModal.value = false;
    }
}

async function handleUpdate() {
    if (!selectedTipo.value) return;
    const request: UpdateTipoVentaRequest = {
        id: selectedTipo.value.id,
        nombre: editNombre.value,
        hacia_donde: editHaciaDonde.value.trim() || undefined,
    };
    const success = await tiposVentaStore.updateTipoVenta(request);
    if (success) {
        showEditModal.value = false;
    }
}

async function handleDelete(id: number) {
    if (await confirm({ message: "¿Está seguro de eliminar este tipo de venta?" })) {
        const success = await tiposVentaStore.deleteTipoVenta(id);
        if (!success) {
            useToasts().error(
                tiposVentaStore.error || "No se pudo eliminar el tipo de venta.",
            );
        }
    }
}
</script>

<template>
    <div class="tipos-venta-page">
        <div class="page-header">
            <h1>Gestión de Tipos de Venta</h1>
            <button
                v-if="canCreateTipoVenta()"
                @click="openCreateModal"
                class="btn-primary"
            >
                Crear Tipo de Venta
            </button>
        </div>

        <div v-if="tiposVentaStore.loading" class="loading">Cargando...</div>

        <div v-if="tiposVentaStore.error" class="error-banner">
            {{ tiposVentaStore.error }}
        </div>

        <table v-if="!tiposVentaStore.loading" class="tipos-table">
            <thead>
                <tr>
                    <th>Nombre</th>
                    <th>Hacia dónde</th>
                    <th>Acciones</th>
                </tr>
            </thead>
            <tbody>
                <tr
                    v-for="tipo in tiposVentaStore.tipos"
                    :key="tipo.id"
                >
                    <td>{{ tipo.nombre }}</td>
                    <td>{{ tipo.hacia_donde || "—" }}</td>
                    <td class="actions">
                        <button
                            v-if="canUpdateTipoVenta()"
                            @click="openEditModal(tipo)"
                            class="btn-icon"
                            title="Editar"
                        >
                            <img src="/svg/edit.svg" alt="Editar" />
                        </button>
                        <button
                            v-if="canDeleteTipoVenta()"
                            @click="handleDelete(tipo.id)"
                            class="btn-icon btn-danger"
                            title="Eliminar"
                        >
                            <img src="/svg/trash.svg" alt="Eliminar" />
                        </button>
                    </td>
                </tr>
            </tbody>
        </table>

        <div v-if="tiposVentaStore.tipos.length === 0" class="empty-state">
            No hay tipos de venta registrados
        </div>

        <div
            v-if="showCreateModal"
            class="modal-overlay"
            @click.self="showCreateModal = false"
        >
            <div class="modal">
                <h2>Crear Tipo de Venta</h2>
                <form @submit.prevent="handleCreate">
                    <div class="form-group">
                        <label>Nombre</label>
                        <input v-model="newNombre" type="text" required />
                    </div>
                    <div class="form-group">
                        <label>Hacia dónde (opcional)</label>
                        <input
                            v-model="newHaciaDonde"
                            type="text"
                            placeholder="Ej: Alias / CBU / QR"
                        />
                    </div>
                    <div v-if="tiposVentaStore.error" class="error-message">
                        {{ tiposVentaStore.error }}
                    </div>
                    <div class="modal-actions">
                        <button
                            type="button"
                            @click="showCreateModal = false"
                            class="btn-secondary"
                        >
                            Cancelar
                        </button>
                        <button type="submit" class="btn-primary">Crear</button>
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
                <h2>Editar Tipo de Venta</h2>
                <form @submit.prevent="handleUpdate">
                    <div class="form-group">
                        <label>Nombre</label>
                        <input v-model="editNombre" type="text" required />
                    </div>
                    <div class="form-group">
                        <label>Hacia dónde (opcional)</label>
                        <input
                            v-model="editHaciaDonde"
                            type="text"
                            placeholder="Ej: Alias / CBU / QR"
                        />
                    </div>
                    <div v-if="tiposVentaStore.error" class="error-message">
                        {{ tiposVentaStore.error }}
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
.tipos-venta-page {
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

.btn-primary {
    background: #667eea;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    cursor: pointer;
}

.btn-primary:hover {
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

.tipos-table {
    width: 100%;
    background: var(--color-surface);
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.tipos-table th,
.tipos-table td {
    padding: 1rem;
    text-align: left;
}

.tipos-table th {
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
    max-width: 400px;
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

.form-group input {
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
</style>
