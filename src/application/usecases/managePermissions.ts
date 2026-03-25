import type { Permission, AddPermissionRequest } from "../../domain/entities";
import type { IUserRepository } from "../../domain/interfaces";

export class ManagePermissionsUseCase {
  constructor(private repository: IUserRepository) {}

  async addPermission(userId: number, permissionId: number): Promise<void> {
    const request: AddPermissionRequest = { user_id: userId, permission_id: permissionId };
    return await this.repository.addPermissionToUser(request);
  }

  async removePermission(userId: number, permissionId: number): Promise<void> {
    const request: AddPermissionRequest = { user_id: userId, permission_id: permissionId };
    return await this.repository.removePermissionFromUser(request);
  }

  async getUserPermissions(userId: number): Promise<Permission[]> {
    return await this.repository.getUserPermissions(userId);
  }

  async getAllPermissions(): Promise<Permission[]> {
    return await this.repository.getAllPermissions();
  }

  async createPermission(name: string): Promise<Permission> {
    return await this.repository.createPermission(name);
  }
}
