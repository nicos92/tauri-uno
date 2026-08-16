import type { User, Permission, UserPermission, LoginRequest, CreateUserRequest, UpdateUserRequest, AddPermissionRequest, ChangePasswordRequest } from "../../domain/entities";

export interface LoginResponse {
  user: User;
  permissions: string[];
}

export interface IUserRepository {
  ensureDbReady(): Promise<void>;
  login(request: LoginRequest): Promise<LoginResponse>;
  createUser(request: CreateUserRequest): Promise<User>;
  getAllUsers(): Promise<User[]>;
  updateUser(request: UpdateUserRequest): Promise<User>;
  deleteUser(id: number): Promise<void>;
  changePassword(request: ChangePasswordRequest): Promise<User>;
  addPermissionToUser(request: AddPermissionRequest): Promise<void>;
  removePermissionFromUser(request: AddPermissionRequest): Promise<void>;
  getUserPermissions(userId: number): Promise<UserPermission[]>;
  getAllPermissions(): Promise<Permission[]>;
  createPermission(name: string): Promise<Permission>;
}
