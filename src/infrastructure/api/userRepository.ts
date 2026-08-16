import { invoke } from "@tauri-apps/api/core";
import type { User, Permission, UserPermission, LoginRequest, CreateUserRequest, UpdateUserRequest, AddPermissionRequest, ChangePasswordRequest } from "../../domain/entities";
import type { IUserRepository } from "../../domain/interfaces";
import { getCurrentUserId } from "../utils/currentUser";

export interface LoginResponse {
  user: User;
  permissions: string[];
}

export class UserApiRepository implements IUserRepository {

  async ensureDbReady(): Promise<void> {
    return await invoke<void>("ensure_db_ready");
  }

  async login(request: LoginRequest): Promise<LoginResponse> {
    return await invoke<LoginResponse>("login", { request });
  }

  async createUser(request: CreateUserRequest): Promise<User> {
    return await invoke<User>("create_user", { userId: getCurrentUserId(), request });
  }

  async getAllUsers(): Promise<User[]> {
    return await invoke<User[]>("get_all_users", { userId: getCurrentUserId() });
  }

  async updateUser(request: UpdateUserRequest): Promise<User> {
    return await invoke<User>("update_user", { userId: getCurrentUserId(), request });
  }

  async deleteUser(id: number): Promise<void> {
    return await invoke<void>("delete_user", { userId: getCurrentUserId(), id });
  }

  async changePassword(request: ChangePasswordRequest): Promise<User> {
    return await invoke<User>("change_password", { userId: getCurrentUserId(), request });
  }

  async addPermissionToUser(request: AddPermissionRequest): Promise<void> {
    return await invoke<void>("add_permission_to_user", { userId: getCurrentUserId(), request });
  }

  async removePermissionFromUser(request: AddPermissionRequest): Promise<void> {
    return await invoke<void>("remove_permission_from_user", { userId: getCurrentUserId(), request });
  }

  async getUserPermissions(userId: number): Promise<UserPermission[]> {
    return await invoke<UserPermission[]>("get_user_permissions", { 
      userId: getCurrentUserId(),
      targetUserId: userId 
    });
  }

  async getAllPermissions(): Promise<Permission[]> {
    return await invoke<Permission[]>("get_all_permissions", { userId: getCurrentUserId() });
  }

  async createPermission(name: string): Promise<Permission> {
    return await invoke<Permission>("create_permission", { userId: getCurrentUserId(), name });
  }
}
