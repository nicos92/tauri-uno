import type {
  Proveedor,
  CreateProveedorRequest,
  UpdateProveedorRequest,
} from "../../domain/entities";
import type { IProveedorRepository } from "../../domain/interfaces";

export class ProveedorUseCase {
  constructor(private repository: IProveedorRepository) {}

  async getAllProveedores(): Promise<Proveedor[]> {
    return await this.repository.getAllProveedores();
  }

  async createProveedor(request: CreateProveedorRequest): Promise<Proveedor> {
    return await this.repository.createProveedor(request);
  }

  async updateProveedor(request: UpdateProveedorRequest): Promise<Proveedor> {
    return await this.repository.updateProveedor(request);
  }

  async deleteProveedor(id: number): Promise<void> {
    return await this.repository.deleteProveedor(id);
  }
}
