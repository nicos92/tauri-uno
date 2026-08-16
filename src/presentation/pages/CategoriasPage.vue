<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useCategoriasStore } from "../stores";
import { usePermissions } from "../composables/usePermissions";
import { useToasts } from "../composables/useToasts";
import PageHeader from "../components/ui/PageHeader.vue";
import DataTable from "../components/ui/DataTable.vue";
import EntityFormModal from "../components/ui/EntityFormModal.vue";
import ConfirmButton from "../components/ui/ConfirmButton.vue";
import type {
    Categoria,
    CreateCategoriaRequest,
    UpdateCategoriaRequest,
} from "../../domain/entities";

const categoriasStore = useCategoriasStore();
const { canCreateCategoria, canUpdateCategoria, canDeleteCategoria } =
    usePermissions();

const showCreateModal = ref(false);
const showEditModal = ref(false);
const selectedCategoria = ref<Categoria | null>(null);

const newCategoria = ref("");
const editCategoria = ref("");

onMounted(async () => {
    await categoriasStore.fetchCategorias();
});

function openCreateModal() {
    newCategoria.value = "";
    showCreateModal.value = true;
}

function openEditModal(categoria: Categoria) {
    selectedCategoria.value = categoria;
    editCategoria.value = categoria.categoria;
    showEditModal.value = true;
}

async function handleCreate() {
    const request: CreateCategoriaRequest = {
        categoria: newCategoria.value,
    };
    const success = await categoriasStore.createCategoria(request);
    if (success) {
        showCreateModal.value = false;
    }
}

async function handleUpdate() {
    if (!selectedCategoria.value) return;
    const request: UpdateCategoriaRequest = {
        id: selectedCategoria.value.id,
        categoria: editCategoria.value,
    };
    const success = await categoriasStore.updateCategoria(request);
    if (success) {
        showEditModal.value = false;
    }
}

async function handleDelete(id: number) {
    const success = await categoriasStore.deleteCategoria(id);
    if (!success) {
        useToasts().error(
            categoriasStore.error || "No se pudo eliminar la categoría.",
        );
    }
}
</script>

<template>
    <div class="categorias-page">
        <PageHeader title="Gestión de Categorías">
            <button
                v-if="canCreateCategoria()"
                @click="openCreateModal"
                class="btn-primary"
            >
                Crear Categoría
            </button>
        </PageHeader>

        <div v-if="categoriasStore.error" class="error-banner">
            {{ categoriasStore.error }}
        </div>

        <DataTable
            :columns="['Categoría', 'Acciones']"
            :loading="categoriasStore.loading"
            :count="categoriasStore.categorias.length"
            empty="No hay categorías registradas"
        >
            <tr v-for="categoria in categoriasStore.categorias" :key="categoria.id">
                <td>{{ categoria.categoria }}</td>
                <td class="actions">
                    <button
                        v-if="canUpdateCategoria()"
                        @click="openEditModal(categoria)"
                        class="btn-icon"
                        title="Editar"
                    >
                        <img src="/svg/edit.svg" alt="Editar" />
                    </button>
                    <ConfirmButton
                        v-if="canDeleteCategoria()"
                        message="¿Está seguro de eliminar esta categoría?"
                        @confirmed="handleDelete(categoria.id)"
                    />
                </td>
            </tr>
        </DataTable>

        <EntityFormModal
            v-model="showCreateModal"
            title="Crear Categoría"
            :error="categoriasStore.error"
            submit-label="Crear"
            max-width="400px"
            @submit="handleCreate"
        >
            <div class="form-group">
                <label>Categoría</label>
                <input v-model="newCategoria" type="text" required />
            </div>
        </EntityFormModal>

        <EntityFormModal
            v-model="showEditModal"
            title="Editar Categoría"
            :error="categoriasStore.error"
            submit-label="Guardar"
            max-width="400px"
            @submit="handleUpdate"
        >
            <div class="form-group">
                <label>Categoría</label>
                <input v-model="editCategoria" type="text" required />
            </div>
        </EntityFormModal>
    </div>
</template>

<style scoped>
.categorias-page {
    padding: 2rem;
    background: var(--color-bg);
    min-height: 100%;
}
</style>
