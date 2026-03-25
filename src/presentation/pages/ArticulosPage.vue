<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useArticulosStore, useSubCategoriasStore, useProveedoresStore, useCategoriasStore } from "../stores";
import { usePermissions } from "../composables/usePermissions";
import type { Articulo, CreateArticuloRequest, UpdateArticuloRequest } from "../../domain/entities";

const articulosStore = useArticulosStore();
const subCategoriasStore = useSubCategoriasStore();
const proveedoresStore = useProveedoresStore();
const categoriasStore = useCategoriasStore();
const { canCreateArticulo, canUpdateArticulo, canDeleteArticulo } = usePermissions();

const showCreateModal = ref(false);
const showEditModal = ref(false);
const selectedArticulo = ref<Articulo | null>(null);

const newArticulo = ref("");
const newCodArticulo = ref("");
const newIdSubCategoria = ref<number | null>(null);
const newIdProveedor = ref<number | null>(null);

const editArticulo = ref("");
const editCodArticulo = ref("");
const editIdSubCategoria = ref<number | null>(null);
const editIdProveedor = ref<number | null>(null);

const articulosCompletos = computed(() => {
  return articulosStore.articulos.map((a) => {
    const subCat = subCategoriasStore.subCategorias.find((s) => s.id === a.id_sub_categoria);
    const cat = subCat ? categoriasStore.categorias.find((c) => c.id === subCat.id_categoria) : null;
    const prov = proveedoresStore.proveedores.find((p) => p.id === a.id_proveedor);
    return {
      ...a,
      subCategoriaNombre: subCat?.sub_categoria || "Sin sub categoría",
      categoriaNombre: cat?.categoria || "Sin categoría",
      proveedorNombre: prov?.proveedor || "Sin proveedor",
    };
  });
});

const subCategoriasConCategoria = computed(() => {
  return subCategoriasStore.subCategorias.map((sc) => {
    const cat = categoriasStore.categorias.find((c) => c.id === sc.id_categoria);
    return {
      ...sc,
      label: `${cat?.categoria || ""} > ${sc.sub_categoria}`,
    };
  });
});

onMounted(async () => {
  await Promise.all([
    articulosStore.fetchArticulos(),
    subCategoriasStore.fetchSubCategorias(),
    categoriasStore.fetchCategorias(),
    proveedoresStore.fetchProveedores(),
  ]);
});

function openCreateModal() {
  newArticulo.value = "";
  newCodArticulo.value = "";
  newIdSubCategoria.value = null;
  newIdProveedor.value = null;
  showCreateModal.value = true;
}

function openEditModal(art: typeof articulosCompletos.value[0]) {
  selectedArticulo.value = art;
  editArticulo.value = art.articulo;
  editCodArticulo.value = art.cod_articulo;
  editIdSubCategoria.value = art.id_sub_categoria;
  editIdProveedor.value = art.id_proveedor;
  showEditModal.value = true;
}

async function handleCreate() {
  if (!newIdSubCategoria.value || !newIdProveedor.value) return;
  const request: CreateArticuloRequest = {
    articulo: newArticulo.value,
    cod_articulo: newCodArticulo.value,
    id_sub_categoria: newIdSubCategoria.value,
    id_proveedor: newIdProveedor.value,
  };
  const success = await articulosStore.createArticulo(request);
  if (success) {
    showCreateModal.value = false;
  }
}

async function handleUpdate() {
  if (!selectedArticulo.value || !editIdSubCategoria.value || !editIdProveedor.value) return;
  const request: UpdateArticuloRequest = {
    id: selectedArticulo.value.id,
    articulo: editArticulo.value,
    cod_articulo: editCodArticulo.value,
    id_sub_categoria: editIdSubCategoria.value,
    id_proveedor: editIdProveedor.value,
  };
  const success = await articulosStore.updateArticulo(request);
  if (success) {
    showEditModal.value = false;
  }
}

async function handleDelete(id: number) {
  if (confirm("¿Está seguro de eliminar este artículo?")) {
    await articulosStore.deleteArticulo(id);
  }
}
</script>

<template>
  <div class="articulos-page">
    <div class="page-header">
      <h1>Gestión de Artículos</h1>
      <button v-if="canCreateArticulo()" @click="openCreateModal" class="btn-primary">
        Crear Artículo
      </button>
    </div>

    <div v-if="articulosStore.loading || subCategoriasStore.loading || proveedoresStore.loading" class="loading">
      Cargando...
    </div>

    <div v-else-if="articulosStore.error" class="error-message">
      {{ articulosStore.error }}
    </div>

    <table v-else class="articulos-table">
      <thead>
        <tr>
          <th>ID</th>
          <th>Código</th>
          <th>Artículo</th>
          <th>Categoría</th>
          <th>Sub Categoría</th>
          <th>Proveedor</th>
          <th>Acciones</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="art in articulosCompletos" :key="art.id">
          <td>{{ art.id }}</td>
          <td>{{ art.cod_articulo }}</td>
          <td>{{ art.articulo }}</td>
          <td>{{ art.categoriaNombre }}</td>
          <td>{{ art.subCategoriaNombre }}</td>
          <td>{{ art.proveedorNombre }}</td>
          <td class="actions">
            <button v-if="canUpdateArticulo()" @click="openEditModal(art)" class="btn-icon" title="Editar">
              ✏️
            </button>
            <button v-if="canDeleteArticulo()" @click="handleDelete(art.id)" class="btn-icon btn-danger" title="Eliminar">
              🗑️
            </button>
          </td>
        </tr>
      </tbody>
    </table>

    <div v-if="articulosStore.articulos.length === 0" class="empty-state">
      No hay artículos registrados
    </div>

    <div v-if="showCreateModal" class="modal-overlay" @click.self="showCreateModal = false">
      <div class="modal">
        <h2>Crear Artículo</h2>
        <form @submit.prevent="handleCreate">
          <div class="form-group">
            <label>Código</label>
            <input v-model="newCodArticulo" type="text" required />
          </div>
          <div class="form-group">
            <label>Artículo</label>
            <input v-model="newArticulo" type="text" required />
          </div>
          <div class="form-group">
            <label>Sub Categoría</label>
            <select v-model="newIdSubCategoria" required>
              <option :value="null" disabled>Seleccione una sub categoría</option>
              <option v-for="sc in subCategoriasConCategoria" :key="sc.id" :value="sc.id">
                {{ sc.label }}
              </option>
            </select>
          </div>
          <div class="form-group">
            <label>Proveedor</label>
            <select v-model="newIdProveedor" required>
              <option :value="null" disabled>Seleccione un proveedor</option>
              <option v-for="prov in proveedoresStore.proveedores" :key="prov.id" :value="prov.id">
                {{ prov.proveedor }}
              </option>
            </select>
          </div>
          <div v-if="articulosStore.error" class="error-message">
            {{ articulosStore.error }}
          </div>
          <div class="modal-actions">
            <button type="button" @click="showCreateModal = false" class="btn-secondary">
              Cancelar
            </button>
            <button type="submit" class="btn-primary" :disabled="!newIdSubCategoria || !newIdProveedor">
              Crear
            </button>
          </div>
        </form>
      </div>
    </div>

    <div v-if="showEditModal" class="modal-overlay" @click.self="showEditModal = false">
      <div class="modal">
        <h2>Editar Artículo</h2>
        <form @submit.prevent="handleUpdate">
          <div class="form-group">
            <label>Código</label>
            <input v-model="editCodArticulo" type="text" required />
          </div>
          <div class="form-group">
            <label>Artículo</label>
            <input v-model="editArticulo" type="text" required />
          </div>
          <div class="form-group">
            <label>Sub Categoría</label>
            <select v-model="editIdSubCategoria" required>
              <option v-for="sc in subCategoriasConCategoria" :key="sc.id" :value="sc.id">
                {{ sc.label }}
              </option>
            </select>
          </div>
          <div class="form-group">
            <label>Proveedor</label>
            <select v-model="editIdProveedor" required>
              <option v-for="prov in proveedoresStore.proveedores" :key="prov.id" :value="prov.id">
                {{ prov.proveedor }}
              </option>
            </select>
          </div>
          <div v-if="articulosStore.error" class="error-message">
            {{ articulosStore.error }}
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
.articulos-page {
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

.articulos-table {
  width: 100%;
  background: white;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.articulos-table th,
.articulos-table td {
  padding: 1rem;
  text-align: left;
}

.articulos-table th {
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
  font-size: 1.2rem;
  padding: 0.25rem;
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
