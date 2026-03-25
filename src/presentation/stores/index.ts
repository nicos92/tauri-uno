import { defineStore } from "pinia";
import { ref } from "vue";
import type { User, Permission, UserPermission } from "../../domain/entities";
import { UserApiRepository } from "../../infrastructure/api";
import { LoginUseCase, CreateUserUseCase, GetAllUsersUseCase, UpdateUserUseCase, DeleteUserUseCase, ManagePermissionsUseCase } from "../../application/usecases";

const repository = new UserApiRepository();

export const useAuthStore = defineStore("auth", () => {
  const user = ref<User | null>(null);
  const permissions = ref<string[]>([]);
  const isAuthenticated = ref(false);
  const error = ref<string | null>(null);

  const loginUseCase = new LoginUseCase(repository);

  function hasPermission(permission: string): boolean {
    return permissions.value.includes(permission);
  }

  async function login(username: string, password: string): Promise<boolean> {
    error.value = null;
    try {
      const response = await loginUseCase.execute(username, password);
      user.value = response.user;
      permissions.value = response.permissions;
      isAuthenticated.value = true;
      localStorage.setItem("currentUser", JSON.stringify(response.user));
      localStorage.setItem("userPermissions", JSON.stringify(response.permissions));
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  function logout() {
    user.value = null;
    permissions.value = [];
    isAuthenticated.value = false;
    localStorage.removeItem("currentUser");
    localStorage.removeItem("userPermissions");
  }

  function loadFromStorage() {
    const storedUser = localStorage.getItem("currentUser");
    const storedPermissions = localStorage.getItem("userPermissions");
    if (storedUser && storedPermissions) {
      user.value = JSON.parse(storedUser);
      permissions.value = JSON.parse(storedPermissions);
      isAuthenticated.value = true;
    }
  }

  return {
    user,
    permissions,
    isAuthenticated,
    error,
    login,
    logout,
    hasPermission,
    loadFromStorage,
  };
});

export const useUsersStore = defineStore("users", () => {
  const users = ref<User[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  const createUserUseCase = new CreateUserUseCase(repository);
  const getAllUsersUseCase = new GetAllUsersUseCase(repository);
  const updateUserUseCase = new UpdateUserUseCase(repository);
  const deleteUserUseCase = new DeleteUserUseCase(repository);

  async function fetchUsers() {
    loading.value = true;
    error.value = null;
    try {
      users.value = await getAllUsersUseCase.execute();
    } catch (e) {
      error.value = e as string;
    } finally {
      loading.value = false;
    }
  }

  async function createUser(username: string, password: string): Promise<boolean> {
    error.value = null;
    try {
      const newUser = await createUserUseCase.execute(username, password);
      users.value.push(newUser);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  async function updateUser(id: number, username: string, active: boolean): Promise<boolean> {
    error.value = null;
    try {
      const updated = await updateUserUseCase.execute(id, username, active);
      const index = users.value.findIndex((u) => u.id === id);
      if (index !== -1) {
        users.value[index] = updated;
      }
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  async function deleteUser(id: number): Promise<boolean> {
    error.value = null;
    try {
      await deleteUserUseCase.execute(id);
      users.value = users.value.filter((u) => u.id !== id);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  return {
    users,
    loading,
    error,
    fetchUsers,
    createUser,
    updateUser,
    deleteUser,
  };
});

export const usePermissionsStore = defineStore("permissions", () => {
  const allPermissions = ref<Permission[]>([]);
  const userPermissions = ref<Map<number, UserPermission[]>>(new Map());
  const loading = ref(false);
  const error = ref<string | null>(null);

  const managePermissionsUseCase = new ManagePermissionsUseCase(repository);

  async function fetchAllPermissions() {
    loading.value = true;
    error.value = null;
    try {
      allPermissions.value = await managePermissionsUseCase.getAllPermissions();
    } catch (e) {
      error.value = e as string;
    } finally {
      loading.value = false;
    }
  }

  async function fetchUserPermissions(userId: number) {
    error.value = null;
    try {
      const perms = await managePermissionsUseCase.getUserPermissions(userId);
      userPermissions.value.set(userId, perms);
    } catch (e) {
      error.value = e as string;
    }
  }

  async function addPermission(userId: number, permissionId: number): Promise<boolean> {
    error.value = null;
    try {
      await managePermissionsUseCase.addPermission(userId, permissionId);
      await fetchUserPermissions(userId);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  async function removePermission(userId: number, permissionId: number): Promise<boolean> {
    error.value = null;
    try {
      await managePermissionsUseCase.removePermission(userId, permissionId);
      await fetchUserPermissions(userId);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  async function createPermission(name: string): Promise<boolean> {
    error.value = null;
    try {
      const newPerm = await managePermissionsUseCase.createPermission(name);
      allPermissions.value.push(newPerm);
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  function getUserPermissions(userId: number): UserPermission[] {
    return userPermissions.value.get(userId) || [];
  }

  return {
    allPermissions,
    userPermissions,
    loading,
    error,
    fetchAllPermissions,
    fetchUserPermissions,
    addPermission,
    removePermission,
    createPermission,
    getUserPermissions,
  };
});
