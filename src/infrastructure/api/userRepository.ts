import { invoke } from "@tauri-apps/api/core";
import type { User, Permission, LoginRequest, CreateUserRequest, UpdateUserRequest, AddPermissionRequest } from "../../domain/entities";
import type { IUserRepository } from "../../domain/interfaces";

export interface LoginResponse {
  user: User;
  permissions: string[];
}

export class UserApiRepository implements IUserRepository {
  private getCurrentUserId(): number {
    const stored = localStorage.getItem("currentUser");
    if (stored) {
      const user = JSON.parse(stored) as User;
      return user.id;
    }
    return 0;
  }

  async login(request: LoginRequest): Promise<LoginResponse> {
    return await invoke<LoginResponse>("login", { request });
  }

  async createUser(request: CreateUserRequest): Promise<User> {
    return await invoke<User>("create_user", { userId: this.getCurrentUserId(), request });
  }

  async getAllUsers(): Promise<User[]> {
    return await invoke<User[]>("get_all_users", { userId: this.getCurrentUserId() });
  }

  async updateUser(request: UpdateUserRequest): Promise<User> {
    return await invoke<User>("update_user", { userId: this.getCurrentUserId(), request });
  }

  async deleteUser(id: number): Promise<void> {
    return await invoke<void>("delete_user", { userId: this.getCurrentUserId(), id });
  }

  async addPermissionToUser(request: AddPermissionRequest): Promise<void> {
    return await invoke<void>("add_permission_to_user", { userId: this.getCurrentUserId(), request });
  }

  async removePermissionFromUser(request: AddPermissionRequest): Promise<void> {
    return await invoke<void>("remove_permission_from_user", { userId: this.getCurrentUserId(), request });
  }

  async getUserPermissions(userId: number): Promise<Permission[]> {
    return await invoke<Permission[]>("get_user_permissions", { 
      userId: this.getCurrentUserId(),
      targetUserId: userId 
    });
  }

  async getAllPermissions(): Promise<Permission[]> {
    return await invoke<Permission[]>("get_all_permissions", { userId: this.getCurrentUserId() });
  }

  async createPermission(name: string): Promise<Permission> {
    return await invoke<Permission>("create_permission", { userId: this.getCurrentUserId(), name });
  }
}
