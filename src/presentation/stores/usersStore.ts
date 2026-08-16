import { defineStore } from "pinia";
import { ref } from "vue";
import type { User, ChangePasswordRequest } from "../../domain/entities";
import { toErrorMessage } from "../../infrastructure/api/errorHandler";
import { userRepository } from "../../infrastructure/di";
import { CreateUserUseCase, GetAllUsersUseCase, UpdateUserUseCase, DeleteUserUseCase, ChangePasswordUseCase } from "../../application/usecases";

const repository = userRepository;

export const useUsersStore = defineStore("users", () => {
  const users = ref<User[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  const createUserUseCase = new CreateUserUseCase(repository);
  const getAllUsersUseCase = new GetAllUsersUseCase(repository);
  const updateUserUseCase = new UpdateUserUseCase(repository);
  const deleteUserUseCase = new DeleteUserUseCase(repository);
  const changePasswordUseCase = new ChangePasswordUseCase(repository);

  async function fetchUsers() {
    loading.value = true;
    error.value = null;
    try {
      users.value = await getAllUsersUseCase.execute();
    } catch (e) {
      error.value = toErrorMessage(e);
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
      error.value = toErrorMessage(e);
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
      error.value = toErrorMessage(e);
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
      error.value = toErrorMessage(e);
      return false;
    }
  }

  async function changePassword(userId: number, newPassword: string): Promise<boolean> {
    error.value = null;
    try {
      const request: ChangePasswordRequest = {
        target_user_id: userId,
        current_password: null,
        new_password: newPassword,
      };
      await changePasswordUseCase.execute(request);
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
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
    changePassword,
  };
});
