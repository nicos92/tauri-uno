<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useSubCategoriasStore, useCategoriasStore } from "../stores";
import { usePermissions } from "../composables/usePermissions";
import { useToasts } from "../composables/useToasts";
import PageHeader from "../components/ui/PageHeader.vue";
import DataTable from "../components/ui/DataTable.vue";
import EntityFormModal from "../components/ui/EntityFormModal.vue";
import ConfirmButton from "../components/ui/ConfirmButton.vue";
import type {
    SubCategoria,
    CreateSubCategoriaRequest,
    UpdateSubCategoriaRequest,
} from "../../domain/entities";

const subCategoriasStore = useSubCategoriasStore();
const categoriasStore = useCategoriasStore();
const { canCreateSubCategoria, canUpdateSubCategoria, canDeleteSubCategoria } =
    usePermissions();

const showCreateModal = ref(false);
const showEditModal = ref(false);
const selectedSubCategoria = ref<SubCategoria | null>(null);

const newSubCategoria = ref("");
const newIdCategoria = ref<number | null>(null);

const editSubCategoria = ref("");
const editIdCategoria = ref<number | null>(null);

const loading = computed(
    () => subCategoriasStore.loading || categoriasStore.loading,
);

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
    const success = await subCategoriasStore.deleteSubCategoria(id);
    if (!success) {
        useToasts().error(
            subCategoriasStore.error || "No se pudo eliminar la sub categoría.",
        );
    }
}
</script>

<template>
    <div class="sub-categorias-page">
        <PageHeader title="Gestión de Sub Categorías">
            <button
                v-if="canCreateSubCategoria()"
                @click="openCreateModal"
                class="btn-primary"
            >
                Crear Sub Categoría
            </button>
        </PageHeader>

        <div v-if="subCategoriasStore.error" class="error-banner">
            {{ subCategoriasStore.error }}
        </div>

        <DataTable
            :columns="['Sub Categoría', 'Categoría', 'Acciones']"
            :loading="loading"
            :count="subCategoriasStore.subCategorias.length"
            empty="No hay sub categorías registradas"
        >
            <tr v-for="subCat in subCategoriasConCategoria" :key="subCat.id">
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
                    <ConfirmButton
                        v-if="canDeleteSubCategoria()"
                        message="¿Está seguro de eliminar esta subcategoría?"
                        @confirmed="handleDelete(subCat.id)"
                    />
                </td>
            </tr>
        </DataTable>

        <EntityFormModal
            v-model="showCreateModal"
            title="Crear Sub Categoría"
            :error="subCategoriasStore.error"
            submit-label="Crear"
            max-width="400px"
            :disable-submit="!newIdCategoria"
            @submit="handleCreate"
        >
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
        </EntityFormModal>

        <EntityFormModal
            v-model="showEditModal"
            title="Editar Sub Categoría"
            :error="subCategoriasStore.error"
            submit-label="Guardar"
            max-width="400px"
            @submit="handleUpdate"
        >
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
                <input v-model="editSubCategoria" type="text" required />
            </div>
        </EntityFormModal>
    </div>
</template>

<style scoped>
.sub-categorias-page {
    padding: 2rem;
    background: var(--color-bg);
    min-height: 100%;
}
</style>
