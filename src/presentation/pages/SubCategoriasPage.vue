<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useSubCategoriasStore, useCategoriasStore } from "../stores";
import { usePermissions } from "../composables/usePermissions";
import type { SubCategoria, CreateSubCategoriaRequest, UpdateSubCategoriaRequest } from "../../domain/entities";

const subCategoriasStore = useSubCategoriasStore();
const categoriasStore = useCategoriasStore();
const { canCreateSubCategoria, canUpdateSubCategoria, canDeleteSubCategoria } = usePermissions();

const showCreateModal = ref(false);
const showEditModal = ref(false);
const selectedSubCategoria = ref<SubCategoria | null>(null);

const newSubCategoria = ref("");
const newIdCategoria = ref<number | null>(null);

const editSubCategoria = ref("");
const editIdCategoria = ref<number | null>(null);

const subCategoriasConCategoria = computed(() => {
  return subCategoriasStore.subCategorias.map((sc) => {
    const cat = categoriasStore.categorias.find((c) => c.id === sc.id_categoria);
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
  if (confirm("¿Está seguro de eliminar esta subcategoría?")) {
    await subCategoriasStore.deleteSubCategoria(id);
  }
}
</script>

<template>
  <div class="sub-categorias-page">
    <div class="page-header">
      <h1>Gestión de Sub Categorías</h1>
      <button v-if="canCreateSubCategoria()" @click="openCreateModal" class="btn-primary">
        Crear Sub Categoría
      </button>
    </div>

    <div v-if="subCategoriasStore.loading || categoriasStore.loading" class="loading">Cargando...</div>

    <div v-else-if="subCategoriasStore.error" class="error-message">
      {{ subCategoriasStore.error }}
    </div>

    <table v-else class="sub-categorias-table">
      <thead>
        <tr>
          <th>Sub Categoría</th>
          <th>Categoría</th>
          <th>Acciones</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="subCat in subCategoriasConCategoria" :key="subCat.id">
          <td>{{ subCat.sub_categoria }}</td>
          <td>{{ subCat.categoriaNombre }}</td>
          <td class="actions">
            <button v-if="canUpdateSubCategoria()" @click="openEditModal(subCat)" class="btn-icon" title="Editar">
              <img src="/svg/edit.svg" alt="Editar" />
            </button>
            <button v-if="canDeleteSubCategoria()" @click="handleDelete(subCat.id)" class="btn-icon btn-danger" title="Eliminar">
              <img src="/svg/trash.svg" alt="Eliminar" />
            </button>
          </td>
        </tr>
      </tbody>
    </table>

    <div v-if="subCategoriasStore.subCategorias.length === 0" class="empty-state">
      No hay sub categorías registradas
    </div>

    <div v-if="showCreateModal" class="modal-overlay" @click.self="showCreateModal = false">
      <div class="modal">
        <h2>Crear Sub Categoría</h2>
        <form @submit.prevent="handleCreate">
          <div class="form-group">
            <label>Categoría</label>
            <select v-model="newIdCategoria" required>
              <option :value="null" disabled>Seleccione una categoría</option>
              <option v-for="cat in categoriasStore.categorias" :key="cat.id" :value="cat.id">
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
            <button type="button" @click="showCreateModal = false" class="btn-secondary">
              Cancelar
            </button>
            <button type="submit" class="btn-primary" :disabled="!newIdCategoria">Crear</button>
          </div>
        </form>
      </div>
    </div>

    <div v-if="showEditModal" class="modal-overlay" @click.self="showEditModal = false">
      <div class="modal">
        <h2>Editar Sub Categoría</h2>
        <form @submit.prevent="handleUpdate">
          <div class="form-group">
            <label>Categoría</label>
            <select v-model="editIdCategoria" required>
              <option v-for="cat in categoriasStore.categorias" :key="cat.id" :value="cat.id">
                {{ cat.categoria }}
              </option>
            </select>
          </div>
          <div class="form-group">
            <label>Sub Categoría</label>
            <input v-model="editSubCategoria" type="text" required />
          </div>
          <div v-if="subCategoriasStore.error" class="error-message">
            {{ subCategoriasStore.error }}
          </div>
          <div class="modal-actions">
            <button type="button" @click="showEditModal = false" class="btn-secondary">
              Cancelar
            </button>
            <button type="submit" class="btn-primary">Guardar</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<style scoped>
.sub-categorias-page {
  padding: 2rem;
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

.btn-primary:hover:not(:disabled) {
  background: #5568d3;
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-secondary {
  background: #e2e8f0;
  color: #333;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 6px;
  cursor: pointer;
}

.sub-categorias-table {
  width: 100%;
  background: white;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.sub-categorias-table th,
.sub-categorias-table td {
  padding: 1rem;
  text-align: left;
}

.sub-categorias-table th {
  background: #f8fafc;
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
  background: white;
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
  border: 1px solid #ddd;
  border-radius: 6px;
  box-sizing: border-box;
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
  color: #666;
}
</style>
