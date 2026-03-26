<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useUsersStore, usePermissionsStore } from "../stores";
import { usePermissions } from "../composables/usePermissions";
import type { User } from "../../domain/entities";

const usersStore = useUsersStore();
const permissionsStore = usePermissionsStore();
const { canCreateUser, canUpdateUser, canDeleteUser, canAssignPermission, canRemovePermission } = usePermissions();

const showCreateModal = ref(false);
const showEditModal = ref(false);
const showPermissionsModal = ref(false);
const selectedUser = ref<User | null>(null);

const newUsername = ref("");
const newPassword = ref("");
const editUsername = ref("");
const editActive = ref(true);

const selectedUserAssignedPermissions = computed(() => {
  return permissionsStore.getUserPermissions(selectedUser.value?.id || 0);
});

const selectedUserAvailablePermissions = computed(() => {
  const assignedIds = new Set(selectedUserAssignedPermissions.value.map(p => p.id));
  return permissionsStore.allPermissions.filter(p => !assignedIds.has(p.id));
});

onMounted(async () => {
  await usersStore.fetchUsers();
  await permissionsStore.fetchAllPermissions();
});

async function handleCreate() {
  const success = await usersStore.createUser(newUsername.value, newPassword.value);
  if (success) {
    showCreateModal.value = false;
    newUsername.value = "";
    newPassword.value = "";
  }
}

function openEditModal(user: User) {
  selectedUser.value = user;
  editUsername.value = user.username;
  editActive.value = user.active;
  showEditModal.value = true;
}

async function handleUpdate() {
  if (!selectedUser.value) return;
  const success = await usersStore.updateUser(selectedUser.value.id, editUsername.value, editActive.value);
  if (success) {
    showEditModal.value = false;
  }
}

async function handleDelete(id: number) {
  if (confirm("¿Está seguro de eliminar este usuario?")) {
    await usersStore.deleteUser(id);
  }
}

async function openPermissionsModal(user: User) {
  selectedUser.value = user;
  await permissionsStore.fetchUserPermissions(user.id);
  showPermissionsModal.value = true;
}

async function addPermission(permissionId: number) {
  if (!selectedUser.value) return;
  await permissionsStore.addPermission(selectedUser.value.id, permissionId);
}

async function removePermission(permissionId: number) {
  if (!selectedUser.value) return;
  await permissionsStore.removePermission(selectedUser.value.id, permissionId);
}
</script>

<template>
  <div class="users-page">
    <div class="page-header">
      <h1>Gestión de Usuarios</h1>
      <button v-if="canCreateUser()" @click="showCreateModal = true" class="btn-primary">
        Crear Usuario
      </button>
    </div>
    
    <div v-if="usersStore.loading" class="loading">Cargando...</div>
    
    <div v-else-if="usersStore.error" class="error-message">
      {{ usersStore.error }}
    </div>
    
    <table v-else class="users-table">
      <thead>
        <tr>
          <th>Usuario</th>
          <th>Activo</th>
          <th>Fecha Creación</th>
          <th>Acciones</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="user in usersStore.users" :key="user.id">
          <td>{{ user.username }}</td>
          <td>
            <span :class="user.active ? 'status-active' : 'status-inactive'">
              {{ user.active ? "Sí" : "No" }}
            </span>
          </td>
          <td>{{ new Date(user.created_at).toLocaleDateString() }}</td>
          <td class="actions">
            <button v-if="canAssignPermission() || canRemovePermission()" @click="openPermissionsModal(user)" class="btn-icon" title="Permisos">
              <img src="/svg/permissions.svg" alt="Permisos" />
            </button>
            <button v-if="canUpdateUser()" @click="openEditModal(user)" class="btn-icon" title="Editar">
              <img src="/svg/edit.svg" alt="Editar" />
            </button>
            <button v-if="canDeleteUser()" @click="handleDelete(user.id)" class="btn-icon btn-danger" title="Eliminar">
              <img src="/svg/trash.svg" alt="Eliminar" />
            </button>
          </td>
        </tr>
      </tbody>
    </table>
    
    <div v-if="usersStore.users.length === 0" class="empty-state">
      No hay usuarios registrados
    </div>

    <div v-if="showCreateModal" class="modal-overlay" @click.self="showCreateModal = false">
      <div class="modal">
        <h2>Crear Usuario</h2>
        <form @submit.prevent="handleCreate">
          <div class="form-group">
            <label>Usuario</label>
            <input v-model="newUsername" type="text" required />
          </div>
          <div class="form-group">
            <label>Contraseña</label>
            <input v-model="newPassword" type="password" required />
          </div>
          <div v-if="usersStore.error" class="error-message">
            {{ usersStore.error }}
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
        <h2>Editar Usuario</h2>
        <form @submit.prevent="handleUpdate">
          <div class="form-group">
            <label>Usuario</label>
            <input v-model="editUsername" type="text" required />
          </div>
          <div class="form-group">
            <label>
              <input v-model="editActive" type="checkbox" />
              Usuario Activo
            </label>
          </div>
          <div v-if="usersStore.error" class="error-message">
            {{ usersStore.error }}
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

    <div v-if="showPermissionsModal" class="modal-overlay" @click.self="showPermissionsModal = false">
      <div class="modal modal-large">
        <h2>Permisos de {{ selectedUser?.username }}</h2>
        <div class="permissions-grid">
          <div class="permission-section">
            <h3>Permisos Asignados</h3>
            <ul class="permission-list">
              <li v-for="perm in selectedUserAssignedPermissions" :key="perm.id">
                <div class="perm-info">
                  <span class="perm-name">{{ perm.permission }}</span>
                  <span class="perm-date">Asignado: {{ new Date(perm.assigned_at).toLocaleString() }}</span>
                </div>
                <button @click="removePermission(perm.id)" class="btn-remove">×</button>
              </li>
              <li v-if="selectedUserAssignedPermissions.length === 0" class="empty">
                Sin permisos asignados
              </li>
            </ul>
          </div>
          <div class="permission-section">
            <h3>Permisos Disponibles</h3>
            <ul class="permission-list">
              <li v-for="perm in selectedUserAvailablePermissions" :key="perm.id">
                {{ perm.permission }}
                <button @click="addPermission(perm.id)" class="btn-add">+</button>
              </li>
              <li v-if="selectedUserAvailablePermissions.length === 0" class="empty">
                Todos los permisos asignados
              </li>
            </ul>
          </div>
        </div>
        <div class="modal-actions">
          <button @click="showPermissionsModal = false" class="btn-secondary">
            Cerrar
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.users-page {
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

.users-table {
  width: 100%;
  background: white;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.users-table th,
.users-table td {
  padding: 1rem;
  text-align: left;
}

.users-table th {
  background: #f8fafc;
  font-weight: 600;
}

.status-active {
  color: #38a169;
  font-weight: 500;
}

.status-inactive {
  color: #e53e3e;
  font-weight: 500;
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

.modal-large {
  max-width: 600px;
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
}

.form-group input[type="text"],
.form-group input[type="password"] {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #ddd;
  border-radius: 6px;
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

.loading, .empty-state {
  text-align: center;
  padding: 2rem;
  color: #666;
}

.permissions-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.permission-section h3 {
  margin: 0 0 0.5rem;
  font-size: 1rem;
}

.permission-list {
  list-style: none;
  padding: 0;
  margin: 0;
  max-height: 200px;
  overflow-y: auto;
}

.permission-list li {
  display: flex;
  justify-content: space-between;
  padding: 0.5rem;
  border-bottom: 1px solid #eee;
}

.permission-list li.empty {
  color: #999;
  font-style: italic;
}

.permission-list li {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.perm-info {
  display: flex;
  flex-direction: column;
}

.perm-name {
  font-weight: 500;
}

.perm-date {
  font-size: 0.75rem;
  color: #888;
}

.btn-add {
  background: #48bb78;
  color: white;
  border: none;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  cursor: pointer;
}

.btn-remove {
  background: #e53e3e;
  color: white;
  border: none;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  cursor: pointer;
}
</style>
