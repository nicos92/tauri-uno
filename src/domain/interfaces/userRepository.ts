import type { User, Permission, LoginRequest, CreateUserRequest, UpdateUserRequest, AddPermissionRequest } from "../../domain/entities";

export interface IUserRepository {
  login(request: LoginRequest): Promise<User>;
  createUser(request: CreateUserRequest): Promise<User>;
  getAllUsers(): Promise<User[]>;
  updateUser(request: UpdateUserRequest): Promise<User>;
  deleteUser(id: number): Promise<void>;
  addPermissionToUser(request: AddPermissionRequest): Promise<void>;
  removePermissionFromUser(request: AddPermissionRequest): Promise<void>;
  getUserPermissions(userId: number): Promise<Permission[]>;
  getAllPermissions(): Promise<Permission[]>;
  createPermission(name: string): Promise<Permission>;
}
