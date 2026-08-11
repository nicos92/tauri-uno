<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useSubCategoriasStore, useCategoriasStore } from "../stores";
import { usePermissions } from "../composables/usePermissions";
import { useToasts } from "../composables/useToasts";
import { useConfirm } from "../composables/useConfirm";
import type {
    SubCategoria,
    CreateSubCategoriaRequest,
    UpdateSubCategoriaRequest,
} from "../../domain/entities";

const subCategoriasStore = useSubCategoriasStore();
const categoriasStore = useCategoriasStore();
const { canCreateSubCategoria, canUpdateSubCategoria, canDeleteSubCategoria } =
    usePermissions();
const { confirm } = useConfirm();

const showCreateModal = ref(false);
const showEditModal = ref(false);
const selectedSubCategoria = ref<SubCategoria | null>(null);

const newSubCategoria = ref("");
const newIdCategoria = ref<number | null>(null);

const editSubCategoria = ref("");
const editIdCategoria = ref<number | null>(null);

const subCategoriasConCategoria = computed(() => {
    return subCategoriasStore.subCategorias.map((sc) => {
        const cat = categoriasStore.categorias.find(
            (c) => c.id === sc.id_categoria,
        );
        return {
            ...sc,
            categoriaNombre: cat?.categoria || "Sin categoría",
        };
    });
});

onMounted(async () => {
    await Promise.all([
        subCategoriasStore.fetchSubCategorias(),
        categoriasStore.fetchCategorias(),
    ]);
});

function openCreateModal() {
    newSubCategoria.value = "";
    newIdCategoria.value = null;
    showCreateModal.value = true;
}

function openEditModal(subCat: SubCategoria) {
    selectedSubCategoria.value = subCat;
    editSubCategoria.value = subCat.sub_categoria;
    editIdCategoria.value = subCat.id_categoria;
    showEditModal.value = true;
}

async function handleCreate() {
    if (!newIdCategoria.value) return;
    const request: CreateSubCategoriaRequest = {
        sub_categoria: newSubCategoria.value,
        id_categoria: newIdCategoria.value,
    };
    const success = await subCategoriasStore.createSubCategoria(request);
    if (success) {
        showCreateModal.value = false;
    }
}

async function handleUpdate() {
    if (!selectedSubCategoria.value || !editIdCategoria.value) return;
    const request: UpdateSubCategoriaRequest = {
        id: selectedSubCategoria.value.id,
        sub_categoria: editSubCategoria.value,
        id_categoria: editIdCategoria.value,
    };
    const success = await subCategoriasStore.updateSubCategoria(request);
    if (success) {
        showEditModal.value = false;
    }
}

async function handleDelete(id: number) {
    if (await confirm({ message: "¿Está seguro de eliminar esta subcategoría?" })) {
        const success = await subCategoriasStore.deleteSubCategoria(id);
        if (!success) {
            useToasts().error(
                subCategoriasStore.error ||
                    "No se pudo eliminar la sub categoría.",
            );
        }
    }
}
</script>

<template>
    <div class="sub-categorias-page">
        <div class="page-header">
            <h1>Gestión de Sub Categorías</h1>
            <button
                v-if="canCreateSubCategoria()"
                @click="openCreateModal"
                class="btn-primary"
            >
                Crear Sub Categoría
            </button>
        </div>

        <div
            v-if="subCategoriasStore.loading || categoriasStore.loading"
            class="loading"
        >
            Cargando...
        </div>

        <div v-if="subCategoriasStore.error" class="error-banner">
            {{ subCategoriasStore.error }}
        </div>

        <div class="table-wrapper">
        <table
            v-if="!(subCategoriasStore.loading || categoriasStore.loading)"
            class="sub-categorias-table"
        >
            <thead>
                <tr>
                    <th>Sub Categoría</th>
                    <th>Categoría</th>
                    <th>Acciones</th>
                </tr>
            </thead>
            <tbody>
                <tr
                    v-for="subCat in subCategoriasConCategoria"
                    :key="subCat.id"
                >
                    <td>{{ subCat.sub_categoria }}</td>
                    <td>{{ subCat.categoriaNombre }}</td>
                    <td class="actions">
                        <button
                            v-if="canUpdateSubCategoria()"
                            @click="openEditModal(subCat)"
                            class="btn-icon"
                            title="Editar"
                        >
                            <img src="/svg/edit.svg" alt="Editar" />
                        </button>
                        <button
                            v-if="canDeleteSubCategoria()"
                            @click="handleDelete(subCat.id)"
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
            v-if="subCategoriasStore.subCategorias.length === 0"
            class="empty-state"
        >
            No hay sub categorías registradas
        </div>

        <div
            v-if="showCreateModal"
            class="modal-overlay"
            @click.self="showCreateModal = false"
        >
            <div class="modal">
                <h2>Crear Sub Categoría</h2>
                <form @submit.prevent="handleCreate">
                    <div class="form-group">
                        <label>Categoría</label>
                        <select v-model="newIdCategoria" required>
                            <option :value="null" disabled>
                                Seleccione una categoría
                            </option>
                            <option
                                v-for="cat in categoriasStore.categorias"
                                :key="cat.id"
                                :value="cat.id"
                            >
                                {{ cat.categoria }}
                            </option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label>Sub Categoría</label>
                        <input v-model="newSubCategoria" type="text" required />
                    </div>
                    <div v-if="subCategoriasStore.error" class="error-message">
                        {{ subCategoriasStore.error }}
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
                            :disabled="!newIdCategoria"
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
                <h2>Editar Sub Categoría</h2>
                <form @submit.prevent="handleUpdate">
                    <div class="form-group">
                        <label>Categoría</label>
                        <select v-model="editIdCategoria" required>
                            <option
                                v-for="cat in categoriasStore.categorias"
                                :key="cat.id"
                                :value="cat.id"
                            >
                                {{ cat.categoria }}
                            </option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label>Sub Categoría</label>
                        <input
                            v-model="editSubCategoria"
                            type="text"
                            required
                        />
                    </div>
                    <div v-if="subCategoriasStore.error" class="error-message">
                        {{ subCategoriasStore.error }}
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
.sub-categorias-page {
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

.table-wrapper {
    overflow-x: auto;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.sub-categorias-table {
    width: 100%;
    background: var(--color-surface);
}

.sub-categorias-table th,
.sub-categorias-table td {
    padding: 1rem;
    text-align: left;
}

.sub-categorias-table th {
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

.form-group select {
    background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='12' height='8' viewBox='0 0 12 8'><path d='M1 1l5 5 5-5' fill='none' stroke='%236b7280' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'/></svg>");
    background-repeat: no-repeat;
    background-position: right 0.75rem center;
    padding-right: 2.5rem;
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
