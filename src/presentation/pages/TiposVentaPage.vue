<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useTiposVentaStore } from "../stores";
import { usePermissions } from "../composables/usePermissions";
import { useToasts } from "../composables/useToasts";
import PageHeader from "../components/ui/PageHeader.vue";
import DataTable from "../components/ui/DataTable.vue";
import EntityFormModal from "../components/ui/EntityFormModal.vue";
import ConfirmButton from "../components/ui/ConfirmButton.vue";
import type {
    CreateTipoVentaRequest,
    TipoVenta,
    UpdateTipoVentaRequest,
} from "../../domain/entities";

const tiposVentaStore = useTiposVentaStore();
const { canCreateTipoVenta, canUpdateTipoVenta, canDeleteTipoVenta } =
    usePermissions();

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
    const success = await tiposVentaStore.deleteTipoVenta(id);
    if (!success) {
        useToasts().error(
            tiposVentaStore.error || "No se pudo eliminar el tipo de venta.",
        );
    }
}
</script>

<template>
    <div class="tipos-venta-page">
        <PageHeader title="Gestión de Tipos de Venta">
            <button
                v-if="canCreateTipoVenta()"
                @click="openCreateModal"
                class="btn-primary"
            >
                Crear Tipo de Venta
            </button>
        </PageHeader>

        <div v-if="tiposVentaStore.error" class="error-banner">
            {{ tiposVentaStore.error }}
        </div>

        <DataTable
            :columns="['Nombre', 'Hacia dónde', 'Acciones']"
            :loading="tiposVentaStore.loading"
            :count="tiposVentaStore.tipos.length"
            empty="No hay tipos de venta registrados"
        >
            <tr v-for="tipo in tiposVentaStore.tipos" :key="tipo.id">
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
                    <ConfirmButton
                        v-if="canDeleteTipoVenta()"
                        message="¿Está seguro de eliminar este tipo de venta?"
                        @confirmed="handleDelete(tipo.id)"
                    />
                </td>
            </tr>
        </DataTable>

        <EntityFormModal
            v-model="showCreateModal"
            title="Crear Tipo de Venta"
            :error="tiposVentaStore.error"
            submit-label="Crear"
            max-width="400px"
            @submit="handleCreate"
        >
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
        </EntityFormModal>

        <EntityFormModal
            v-model="showEditModal"
            title="Editar Tipo de Venta"
            :error="tiposVentaStore.error"
            submit-label="Guardar"
            max-width="400px"
            @submit="handleUpdate"
        >
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
        </EntityFormModal>
    </div>
</template>

<style scoped>
.tipos-venta-page {
    padding: 2rem;
    background: var(--color-bg);
    min-height: 100%;
}
</style>
