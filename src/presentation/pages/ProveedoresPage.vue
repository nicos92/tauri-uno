<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useProveedoresStore } from "../stores";
import { usePermissions } from "../composables/usePermissions";
import type {
    Proveedor,
    CreateProveedorRequest,
    UpdateProveedorRequest,
} from "../../domain/entities";

const proveedoresStore = useProveedoresStore();
const { canCreateProveedor, canUpdateProveedor, canDeleteProveedor } =
    usePermissions();

const showCreateModal = ref(false);
const showEditModal = ref(false);
const selectedProveedor = ref<Proveedor | null>(null);

const newProveedor = ref("");
const newNombre = ref("");
const newCuit = ref("");
const newTel = ref("");
const newEmail = ref("");
const newObservacion = ref("");

const editProveedor = ref("");
const editNombre = ref("");
const editCuit = ref("");
const editTel = ref("");
const editEmail = ref("");
const editObservacion = ref("");

onMounted(async () => {
    await proveedoresStore.fetchProveedores();
});

function openCreateModal() {
    newProveedor.value = "";
    newNombre.value = "";
    newCuit.value = "";
    newTel.value = "";
    newEmail.value = "";
    newObservacion.value = "";
    showCreateModal.value = true;
}

function openEditModal(proveedor: Proveedor) {
    selectedProveedor.value = proveedor;
    editProveedor.value = proveedor.proveedor;
    editNombre.value = proveedor.nombre;
    editCuit.value = proveedor.cuit || "";
    editTel.value = proveedor.tel || "";
    editEmail.value = proveedor.email || "";
    editObservacion.value = proveedor.observacion || "";
    showEditModal.value = true;
}

async function handleCreate() {
    const request: CreateProveedorRequest = {
        proveedor: newProveedor.value,
        nombre: newNombre.value,
        cuit: newCuit.value.toString() || undefined,
        tel: newTel.value.toString() || undefined,
        email: newEmail.value || undefined,
        observacion: newObservacion.value || undefined,
    };
    const success = await proveedoresStore.createProveedor(request);
    if (success) {
        showCreateModal.value = false;
    }
}

async function handleUpdate() {
    if (!selectedProveedor.value) return;
    const request: UpdateProveedorRequest = {
        id: selectedProveedor.value.id,
        proveedor: editProveedor.value,
        nombre: editNombre.value,
        cuit: editCuit.value.toString() || undefined,
        tel: editTel.value.toString() || undefined,
        email: editEmail.value || undefined,
        observacion: editObservacion.value || undefined,
    };
    const success = await proveedoresStore.updateProveedor(request);
    if (success) {
        showEditModal.value = false;
    }
}

async function handleDelete(id: number) {
    if (confirm("¿Está seguro de eliminar este proveedor?")) {
        await proveedoresStore.deleteProveedor(id);
    }
}
</script>

<template>
    <div class="proveedores-page">
        <div class="page-header">
            <h1>Gestión de Proveedores</h1>
            <button
                v-if="canCreateProveedor()"
                @click="openCreateModal"
                class="btn-primary"
            >
                Crear Proveedor
            </button>
        </div>

        <div v-if="proveedoresStore.loading" class="loading">Cargando...</div>

        <div v-else-if="proveedoresStore.error" class="error-message">
            {{ proveedoresStore.error }}
        </div>

        <table v-else class="proveedores-table">
            <thead>
                <tr>
                    <th>Razón Social</th>
                    <th>Nombre</th>
                    <th>CUIT</th>
                    <th>Teléfono</th>
                    <th>Email</th>
                    <th>Acciones</th>
                </tr>
            </thead>
            <tbody>
                <tr
                    v-for="proveedor in proveedoresStore.proveedores"
                    :key="proveedor.id"
                >
                    <td>{{ proveedor.proveedor }}</td>
                    <td>{{ proveedor.nombre }}</td>
                    <td>{{ proveedor.cuit || "-" }}</td>
                    <td>{{ proveedor.tel || "-" }}</td>
                    <td>{{ proveedor.email || "-" }}</td>
                    <td class="actions">
                        <button
                            v-if="canUpdateProveedor()"
                            @click="openEditModal(proveedor)"
                            class="btn-icon"
                            title="Editar"
                        >
                            <img src="/svg/edit.svg" alt="Editar" />
                        </button>
                        <button
                            v-if="canDeleteProveedor()"
                            @click="handleDelete(proveedor.id)"
                            class="btn-icon btn-danger"
                            title="Eliminar"
                        >
                            <img src="/svg/trash.svg" alt="Eliminar" />
                        </button>
                    </td>
                </tr>
            </tbody>
        </table>

        <div
            v-if="proveedoresStore.proveedores.length === 0"
            class="empty-state"
        >
            No hay proveedores registrados
        </div>

        <div
            v-if="showCreateModal"
            class="modal-overlay"
            @click.self="showCreateModal = false"
        >
            <div class="modal">
                <h2>Crear Proveedor</h2>
                <form @submit.prevent="handleCreate">
                    <div class="form-group">
                        <label>Razón Social</label>
                        <input v-model="newProveedor" type="text" required />
                    </div>
                    <div class="form-group">
                        <label>Nombre</label>
                        <input v-model="newNombre" type="text" required />
                    </div>
                    <div class="form-group">
                        <label>CUIT</label>
                        <input v-model="newCuit" type="number" maxlength="11" />
                    </div>
                    <div class="form-group">
                        <label>Teléfono</label>
                        <input v-model="newTel" type="number" />
                    </div>
                    <div class="form-group">
                        <label>Email</label>
                        <input v-model="newEmail" type="email" />
                    </div>
                    <div class="form-group">
                        <label>Observación</label>
                        <textarea v-model="newObservacion" rows="3"></textarea>
                    </div>
                    <div v-if="proveedoresStore.error" class="error-message">
                        {{ proveedoresStore.error }}
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
                <h2>Editar Proveedor</h2>
                <form @submit.prevent="handleUpdate">
                    <div class="form-group">
                        <label>Razón Social</label>
                        <input v-model="editProveedor" type="text" required />
                    </div>
                    <div class="form-group">
                        <label>Nombre</label>
                        <input v-model="editNombre" type="text" required />
                    </div>
                    <div class="form-group">
                        <label>CUIT</label>
                        <input
                            v-model="editCuit"
                            type="number"
                            maxlength="11"
                        />
                    </div>
                    <div class="form-group">
                        <label>Teléfono</label>
                        <input v-model="editTel" type="number" />
                    </div>
                    <div class="form-group">
                        <label>Email</label>
                        <input v-model="editEmail" type="email" />
                    </div>
                    <div class="form-group">
                        <label>Observación</label>
                        <textarea v-model="editObservacion" rows="3"></textarea>
                    </div>
                    <div v-if="proveedoresStore.error" class="error-message">
                        {{ proveedoresStore.error }}
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
.proveedores-page {
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

.proveedores-table {
    width: 100%;
    background: var(--color-surface);
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.proveedores-table th,
.proveedores-table td {
    padding: 1rem;
    text-align: left;
}

.proveedores-table th {
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
.form-group textarea {
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

.loading,
.empty-state {
    text-align: center;
    padding: 2rem;
    color: var(--color-text-muted);
}
</style>
