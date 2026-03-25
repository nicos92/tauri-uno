import { invoke } from "@tauri-apps/api/core";
import type { User, Permission, LoginRequest, CreateUserRequest, UpdateUserRequest, AddPermissionRequest } from "../../domain/entities";
import type { IUserRepository } from "../../domain/interfaces";

export class UserApiRepository implements IUserRepository {
  async login(request: LoginRequest): Promise<User> {
    return await invoke<User>("login", { request });
  }

  async createUser(request: CreateUserRequest): Promise<User> {
    return await invoke<User>("create_user", { request });
  }

  async getAllUsers(): Promise<User[]> {
    return await invoke<User[]>("get_all_users");
  }

  async updateUser(request: UpdateUserRequest): Promise<User> {
    return await invoke<User>("update_user", { request });
  }

  async deleteUser(id: number): Promise<void> {
    return await invoke<void>("delete_user", { id });
  }

  async addPermissionToUser(request: AddPermissionRequest): Promise<void> {
    return await invoke<void>("add_permission_to_user", { request });
  }

  async removePermissionFromUser(request: AddPermissionRequest): Promise<void> {
    return await invoke<void>("remove_permission_from_user", { request });
  }

  async getUserPermissions(userId: number): Promise<Permission[]> {
    return await invoke<Permission[]>("get_user_permissions", { userId });
  }

  async getAllPermissions(): Promise<Permission[]> {
    return await invoke<Permission[]>("get_all_permissions");
  }

  async createPermission(name: string): Promise<Permission> {
    return await invoke<Permission>("create_permission", { name });
  }
}
