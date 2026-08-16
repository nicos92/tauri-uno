import { defineStore } from "pinia";
import { ref } from "vue";
import type { Permission, UserPermission } from "../../domain/entities";
import { toErrorMessage } from "../../infrastructure/api/errorHandler";
import { UserApiRepository } from "../../infrastructure/api";
import { ManagePermissionsUseCase } from "../../application/usecases";

const repository = new UserApiRepository();

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
      error.value = toErrorMessage(e);
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
      error.value = toErrorMessage(e);
    }
  }

  async function addPermission(userId: number, permissionId: number): Promise<boolean> {
    error.value = null;
    try {
      await managePermissionsUseCase.addPermission(userId, permissionId);
      await fetchUserPermissions(userId);
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
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
      error.value = toErrorMessage(e);
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
      error.value = toErrorMessage(e);
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
