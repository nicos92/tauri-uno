<script setup lang="ts">
import { ref, onMounted } from "vue";
import { usePermissionsStore } from "../stores";
import { usePermissions } from "../composables/usePermissions";

const permissionsStore = usePermissionsStore();
const { canCreateCategoria } = usePermissions();

const newPermissionName = ref("");

onMounted(async () => {
  await permissionsStore.fetchAllPermissions();
});

async function handleCreatePermission() {
  if (!newPermissionName.value.trim()) return;
  const success = await permissionsStore.createPermission(newPermissionName.value.trim());
  if (success) {
    newPermissionName.value = "";
  }
}
</script>

<template>
  <div class="permissions-page">
    <div class="page-header">
      <h1>Gestión de Permisos</h1>
    </div>
    
    <div v-if="canCreateCategoria()" class="create-section">
      <h3>Crear Nueva Categoria</h3>
      <form @submit.prevent="handleCreatePermission" class="create-form">
        <input
          v-model="newPermissionName"
          type="text"
          placeholder="Nombre del permiso"
        />
        <button type="submit" class="btn-primary" :disabled="!newPermissionName.trim()">
          Crear
        </button>
      </form>
      <div v-if="permissionsStore.error" class="error-message">
        {{ permissionsStore.error }}
      </div>
    </div>
    
    <div class="permissions-list">
      <h3>Permisos Existentes</h3>
      <div v-if="permissionsStore.loading" class="loading">Cargando...</div>
      <ul v-else>
        <li v-for="perm in permissionsStore.allPermissions" :key="perm.id">
          <span class="permission-name">{{ perm.permission }}</span>
          <span class="permission-date">
            Creado: {{ new Date(perm.created).toLocaleDateString() }}
          </span>
        </li>
        <li v-if="permissionsStore.allPermissions.length === 0" class="empty">
          No hay permisos registrados
        </li>
      </ul>
    </div>
  </div>
</template>

<style scoped>
.permissions-page {
  padding: 2rem;
}

.page-header {
  margin-bottom: 2rem;
}

.page-header h1 {
  margin: 0;
}

.create-section {
  background: white;
  padding: 1.5rem;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  margin-bottom: 2rem;
}

.create-section h3 {
  margin: 0 0 1rem;
}

.create-form {
  display: flex;
  gap: 1rem;
}

.create-form input {
  flex: 1;
  padding: 0.75rem;
  border: 1px solid #ddd;
  border-radius: 6px;
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

.permissions-list {
  background: white;
  padding: 1.5rem;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.permissions-list h3 {
  margin: 0 0 1rem;
}

.permissions-list ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.permissions-list li {
  display: flex;
  justify-content: space-between;
  padding: 0.75rem 0;
  border-bottom: 1px solid #eee;
}

.permissions-list li:last-child {
  border-bottom: none;
}

.permission-name {
  font-weight: 500;
}

.permission-date {
  color: #666;
  font-size: 0.9rem;
}

.permissions-list li.empty {
  color: #999;
  font-style: italic;
}

.error-message {
  color: #e53e3e;
  margin-top: 1rem;
}

.loading {
  text-align: center;
  padding: 1rem;
  color: #666;
}
</style>
