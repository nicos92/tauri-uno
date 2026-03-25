<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useCategoriasStore } from "../stores";
import { usePermissions } from "../composables/usePermissions";
import type { Categoria, CreateCategoriaRequest, UpdateCategoriaRequest } from "../../domain/entities";

const categoriasStore = useCategoriasStore();
const { canCreateCategoria, canUpdateCategoria, canDeleteCategoria } = usePermissions();

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
  if (confirm("¿Está seguro de eliminar esta categoría?")) {
    await categoriasStore.deleteCategoria(id);
  }
}
</script>

<template>
  <div class="categorias-page">
    <div class="page-header">
      <h1>Gestión de Categorías</h1>
      <button v-if="canCreateCategoria()" @click="openCreateModal" class="btn-primary">
        Crear Categoría
      </button>
    </div>

    <div v-if="categoriasStore.loading" class="loading">Cargando...</div>

    <div v-else-if="categoriasStore.error" class="error-message">
      {{ categoriasStore.error }}
    </div>

    <table v-else class="categorias-table">
      <thead>
        <tr>
          <th>ID</th>
          <th>Categoría</th>
          <th>Acciones</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="categoria in categoriasStore.categorias" :key="categoria.id">
          <td>{{ categoria.id }}</td>
          <td>{{ categoria.categoria }}</td>
          <td class="actions">
            <button v-if="canUpdateCategoria()" @click="openEditModal(categoria)" class="btn-icon" title="Editar">
              <img src="/svg/edit.svg" alt="Editar" />
            </button>
            <button v-if="canDeleteCategoria()" @click="handleDelete(categoria.id)" class="btn-icon btn-danger" title="Eliminar">
              <img src="/svg/trash.svg" alt="Eliminar" />
            </button>
          </td>
        </tr>
      </tbody>
    </table>

    <div v-if="categoriasStore.categorias.length === 0" class="empty-state">
      No hay categorías registradas
    </div>

    <div v-if="showCreateModal" class="modal-overlay" @click.self="showCreateModal = false">
      <div class="modal">
        <h2>Crear Categoría</h2>
        <form @submit.prevent="handleCreate">
          <div class="form-group">
            <label>Categoría</label>
            <input v-model="newCategoria" type="text" required />
          </div>
          <div v-if="categoriasStore.error" class="error-message">
            {{ categoriasStore.error }}
          </div>
          <div class="modal-actions">
            <button type="button" @click="showCreateModal = false" class="btn-secondary">
              Cancelar
            </button>
            <button type="submit" class="btn-primary">Crear</button>
          </div>
        </form>
      </div>
    </div>

    <div v-if="showEditModal" class="modal-overlay" @click.self="showEditModal = false">
      <div class="modal">
        <h2>Editar Categoría</h2>
        <form @submit.prevent="handleUpdate">
          <div class="form-group">
            <label>Categoría</label>
            <input v-model="editCategoria" type="text" required />
          </div>
          <div v-if="categoriasStore.error" class="error-message">
            {{ categoriasStore.error }}
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
.categorias-page {
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

.btn-primary:hover {
  background: #5568d3;
}

.btn-secondary {
  background: #e2e8f0;
  color: #333;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 6px;
  cursor: pointer;
}

.categorias-table {
  width: 100%;
  background: white;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.categorias-table th,
.categorias-table td {
  padding: 1rem;
  text-align: left;
}

.categorias-table th {
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

.form-group input {
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
