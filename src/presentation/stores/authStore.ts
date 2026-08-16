import { defineStore } from "pinia";
import { ref } from "vue";
import type { User, ChangePasswordRequest } from "../../domain/entities";
import { toErrorMessage } from "../../infrastructure/api/errorHandler";
import { userRepository } from "../../infrastructure/di";
import { LoginUseCase, ChangePasswordUseCase } from "../../application/usecases";

const repository = userRepository;

export const useAuthStore = defineStore("auth", () => {
  const user = ref<User | null>(null);
  const permissions = ref<string[]>([]);
  const isAuthenticated = ref(false);
  const error = ref<string | null>(null);

  const loginUseCase = new LoginUseCase(repository);
  const changePasswordUseCase = new ChangePasswordUseCase(repository);

  async function ensureDbReady(): Promise<void> {
    await repository.ensureDbReady();
  }

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
      sessionStorage.setItem("currentUser", JSON.stringify(response.user));
      sessionStorage.setItem("userPermissions", JSON.stringify(response.permissions));
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  function logout() {
    user.value = null;
    permissions.value = [];
    isAuthenticated.value = false;
    sessionStorage.removeItem("currentUser");
    sessionStorage.removeItem("userPermissions");
  }

  async function changeOwnPassword(currentPassword: string, newPassword: string): Promise<boolean> {
    error.value = null;
    if (!user.value) return false;
    try {
      const request: ChangePasswordRequest = {
        target_user_id: user.value.id,
        current_password: currentPassword,
        new_password: newPassword,
      };
      await changePasswordUseCase.execute(request);
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  function loadFromStorage() {
    const storedUser = sessionStorage.getItem("currentUser");
    const storedPermissions = sessionStorage.getItem("userPermissions");
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
    changeOwnPassword,
    hasPermission,
    loadFromStorage,
    ensureDbReady,
  };
});
