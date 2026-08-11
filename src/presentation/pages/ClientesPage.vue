<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useClientesStore } from "../stores";
import { usePermissions } from "../composables/usePermissions";
import { useToasts } from "../composables/useToasts";
import { useConfirm } from "../composables/useConfirm";
import type {
    Cliente,
    CreateClienteRequest,
    UpdateClienteRequest,
} from "../../domain/entities";

const clientesStore = useClientesStore();
const { canCreateCliente, canUpdateCliente, canDeleteCliente } =
    usePermissions();
const { confirm } = useConfirm();

const showCreateModal = ref(false);
const showEditModal = ref(false);
const selectedCliente = ref<Cliente | null>(null);

const newNombre = ref("");
const newApellido = ref("");
const newTelefono = ref("");
const newEmail = ref("");
const newDireccion = ref("");

const editNombre = ref("");
const editApellido = ref("");
const editTelefono = ref("");
const editEmail = ref("");
const editDireccion = ref("");

const newFormInvalid = computed(
    () =>
        !newNombre.value.trim() &&
        !newApellido.value.trim() &&
        !newTelefono.value.trim() &&
        !newEmail.value.trim() &&
        !newDireccion.value.trim(),
);

const editFormInvalid = computed(
    () =>
        !editNombre.value.trim() &&
        !editApellido.value.trim() &&
        !editTelefono.value.trim() &&
        !editEmail.value.trim() &&
        !editDireccion.value.trim(),
);

onMounted(async () => {
    await clientesStore.fetchClientes();
});

function isDefaultClient(cliente: Cliente): boolean {
    return cliente.nombre === "Consumidor" && cliente.apellido === "Final";
}

function openCreateModal() {
    newNombre.value = "";
    newApellido.value = "";
    newTelefono.value = "";
    newEmail.value = "";
    newDireccion.value = "";
    showCreateModal.value = true;
}

function openEditModal(cliente: Cliente) {
    selectedCliente.value = cliente;
    editNombre.value = cliente.nombre || "";
    editApellido.value = cliente.apellido || "";
    editTelefono.value = cliente.telefono || "";
    editEmail.value = cliente.email || "";
    editDireccion.value = cliente.direccion || "";
    showEditModal.value = true;
}

async function handleCreate() {
    const request: CreateClienteRequest = {
        nombre: newNombre.value.trim() || undefined,
        apellido: newApellido.value.trim() || undefined,
        telefono: newTelefono.value.trim() || undefined,
        email: newEmail.value.trim() || undefined,
        direccion: newDireccion.value.trim() || undefined,
    };
    const success = await clientesStore.crearCliente(request);
    if (success) {
        showCreateModal.value = false;
    }
}

async function handleUpdate() {
    if (!selectedCliente.value) return;
    const request: UpdateClienteRequest = {
        id: selectedCliente.value.id,
        nombre: editNombre.value.trim() || undefined,
        apellido: editApellido.value.trim() || undefined,
        telefono: editTelefono.value.trim() || undefined,
        email: editEmail.value.trim() || undefined,
        direccion: editDireccion.value.trim() || undefined,
    };
    const success = await clientesStore.actualizarCliente(request);
    if (success) {
        showEditModal.value = false;
    }
}

async function handleDelete(cliente: Cliente) {
    if (isDefaultClient(cliente)) {
        useToasts().error(
            "No se puede eliminar el cliente 'Consumidor Final'.",
        );
        return;
    }
    if (await confirm({ message: "¿Está seguro de eliminar este cliente?" })) {
        const success = await clientesStore.eliminarCliente(cliente.id);
        if (!success) {
            useToasts().error(
                clientesStore.error || "No se pudo eliminar el cliente.",
            );
        }
    }
}

function clienteLabel(cliente: Cliente): string {
    const name = [cliente.nombre, cliente.apellido]
        .filter(Boolean)
        .join(" ");
    return name || cliente.telefono || cliente.email || "Sin datos";
}
</script>

<template>
    <div class="clientes-page">
        <div class="page-header">
            <h1>Gestión de Clientes</h1>
            <button
                v-if="canCreateCliente()"
                @click="openCreateModal"
                class="btn-primary"
            >
                Crear Cliente
            </button>
        </div>

        <div v-if="clientesStore.loading" class="loading">Cargando...</div>

        <div v-if="clientesStore.error" class="error-banner">
            {{ clientesStore.error }}
        </div>

        <div class="table-wrapper">
        <table v-if="!clientesStore.loading" class="clientes-table">
            <thead>
                <tr>
                    <th>Cliente</th>
                    <th>Teléfono</th>
                    <th>Email</th>
                    <th>Dirección</th>
                    <th>Acciones</th>
                </tr>
            </thead>
            <tbody>
                <tr
                    v-for="cliente in clientesStore.clientes"
                    :key="cliente.id"
                >
                    <td>
                        {{ clienteLabel(cliente) }}
                        <span
                            v-if="isDefaultClient(cliente)"
                            class="default-badge"
                        >
                            por defecto
                        </span>
                    </td>
                    <td>{{ cliente.telefono || "-" }}</td>
                    <td>{{ cliente.email || "-" }}</td>
                    <td>{{ cliente.direccion || "-" }}</td>
                    <td class="actions">
                        <button
                            v-if="canUpdateCliente()"
                            @click="openEditModal(cliente)"
                            class="btn-icon"
                            title="Editar"
                        >
                            <img src="/svg/edit.svg" alt="Editar" />
                        </button>
                        <button
                            v-if="canDeleteCliente() && !isDefaultClient(cliente)"
                            @click="handleDelete(cliente)"
                            class="btn-icon btn-danger"
                            title="Eliminar"
                        >
                            <img src="/svg/trash.svg" alt="Eliminar" />
                        </button>
                    </td>
                </tr>
            </tbody>
        </table>
        </div>

        <div
            v-if="clientesStore.clientes.length === 0"
            class="empty-state"
        >
            No hay clientes registrados
        </div>

        <div
            v-if="showCreateModal"
            class="modal-overlay"
            @click.self="showCreateModal = false"
        >
            <div class="modal">
                <h2>Crear Cliente</h2>
                <form @submit.prevent="handleCreate">
                    <div class="form-group">
                        <label>Nombre</label>
                        <input v-model="newNombre" type="text" />
                    </div>
                    <div class="form-group">
                        <label>Apellido</label>
                        <input v-model="newApellido" type="text" />
                    </div>
                    <div class="form-group">
                        <label>Teléfono</label>
                        <input v-model="newTelefono" type="text" />
                    </div>
                    <div class="form-group">
                        <label>Email</label>
                        <input v-model="newEmail" type="email" />
                    </div>
                    <div class="form-group">
                        <label>Dirección</label>
                        <input v-model="newDireccion" type="text" />
                    </div>
                    <div v-if="newFormInvalid" class="validation-hint">
                        El cliente debe contar al menos con un dato de contacto
                        o identificación.
                    </div>
                    <div v-if="clientesStore.error" class="error-message">
                        {{ clientesStore.error }}
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
                            :disabled="newFormInvalid"
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
                <h2>Editar Cliente</h2>
                <form @submit.prevent="handleUpdate">
                    <div class="form-group">
                        <label>Nombre</label>
                        <input v-model="editNombre" type="text" />
                    </div>
                    <div class="form-group">
                        <label>Apellido</label>
                        <input v-model="editApellido" type="text" />
                    </div>
                    <div class="form-group">
                        <label>Teléfono</label>
                        <input v-model="editTelefono" type="text" />
                    </div>
                    <div class="form-group">
                        <label>Email</label>
                        <input v-model="editEmail" type="email" />
                    </div>
                    <div class="form-group">
                        <label>Dirección</label>
                        <input v-model="editDireccion" type="text" />
                    </div>
                    <div v-if="editFormInvalid" class="validation-hint">
                        El cliente debe contar al menos con un dato de contacto
                        o identificación.
                    </div>
                    <div v-if="clientesStore.error" class="error-message">
                        {{ clientesStore.error }}
                    </div>
                    <div class="modal-actions">
                        <button
                            type="button"
                            @click="showEditModal = false"
                            class="btn-secondary"
                        >
                            Cancelar
                        </button>
                        <button
                            type="submit"
                            class="btn-primary"
                            :disabled="editFormInvalid"
                        >
                            Guardar
                        </button>
                    </div>
                </form>
            </div>
        </div>
    </div>
</template>

<style scoped>
.clientes-page {
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
    background: #3F2281;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    cursor: pointer;
}

.btn-primary:hover {
    background: #5568d3;
}

.btn-primary:disabled {
    background: #a5b4fc;
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

.table-wrapper {
    overflow-x: auto;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.clientes-table {
    width: 100%;
    background: var(--color-surface);
}

.clientes-table th,
.clientes-table td {
    padding: 1rem;
    text-align: left;
}

.clientes-table th {
    background: var(--color-surface-2);
    font-weight: 600;
}

.default-badge {
    display: inline-block;
    margin-left: 0.5rem;
    padding: 0.15rem 0.5rem;
    font-size: 0.7rem;
    border-radius: 999px;
    background: rgba(102, 126, 234, 0.15);
    color: #3F2281;
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

.form-group input {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid var(--color-border);
    border-radius: 6px;
    box-sizing: border-box;
    background: var(--color-surface);
    color: var(--color-text);
}

.validation-hint {
    color: #b7791f;
    background: rgba(183, 121, 31, 0.1);
    border: 1px solid rgba(183, 121, 31, 0.3);
    padding: 0.5rem 0.75rem;
    border-radius: 6px;
    margin-bottom: 1rem;
    font-size: 0.85rem;
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
