import type { User, Permission, UserPermission, LoginRequest, CreateUserRequest, UpdateUserRequest, AddPermissionRequest } from "../../domain/entities";

export interface LoginResponse {
  user: User;
  permissions: string[];
}

export interface IUserRepository {
  login(request: LoginRequest): Promise<LoginResponse>;
  createUser(request: CreateUserRequest): Promise<User>;
  getAllUsers(): Promise<User[]>;
  updateUser(request: UpdateUserRequest): Promise<User>;
  deleteUser(id: number): Promise<void>;
  addPermissionToUser(request: AddPermissionRequest): Promise<void>;
  removePermissionFromUser(request: AddPermissionRequest): Promise<void>;
  getUserPermissions(userId: number): Promise<UserPermission[]>;
  getAllPermissions(): Promise<Permission[]>;
  createPermission(name: string): Promise<Permission>;
}
