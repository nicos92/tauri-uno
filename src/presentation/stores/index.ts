import { defineStore } from "pinia";
import { ref } from "vue";
import type { User } from "../../domain/entities";
import { UserApiRepository } from "../../infrastructure/api";
import { LoginUseCase, CreateUserUseCase, GetAllUsersUseCase, UpdateUserUseCase, DeleteUserUseCase, ManagePermissionsUseCase } from "../../application/usecases";

const repository = new UserApiRepository();

export const useAuthStore = defineStore("auth", () => {
  const user = ref<User | null>(null);
  const isAuthenticated = ref(false);
  const error = ref<string | null>(null);

  const loginUseCase = new LoginUseCase(repository);

  async function login(username: string, password: string): Promise<boolean> {
    error.value = null;
    try {
      user.value = await loginUseCase.execute(username, password);
      isAuthenticated.value = true;
      return true;
    } catch (e) {
      error.value = e as string;
      return false;
    }
  }

  function logout() {
    user.value = null;
    isAuthenticated.value = false;
  }

  return {
    user,
    isAuthenticated,
    error,
    login,
    logout,
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
  const allPermissions = ref<{ id: number; permission: string; created: string }[]>([]);
  const userPermissions = ref<Map<number, { id: number; permission: string; created: string }[]>>(new Map());
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

  function getUserPermissions(userId: number) {
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
