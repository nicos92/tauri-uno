import { invoke } from "@tauri-apps/api/core";
import type { Proveedor, CreateProveedorRequest, UpdateProveedorRequest } from "../../domain/entities";

export class ProveedorApiRepository {
  private getCurrentUserId(): number {
    const stored = localStorage.getItem("currentUser");
    if (stored) {
      const user = JSON.parse(stored);
      return user.id;
    }
    return 0;
  }

  async getAllProveedores(): Promise<Proveedor[]> {
    return await invoke<Proveedor[]>("get_all_proveedores", {
      userId: this.getCurrentUserId(),
    });
  }

  async getProveedorById(id: number): Promise<Proveedor> {
    return await invoke<Proveedor>("get_proveedor_by_id", {
      userId: this.getCurrentUserId(),
      id,
    });
  }

  async createProveedor(request: CreateProveedorRequest): Promise<Proveedor> {
    return await invoke<Proveedor>("create_proveedor", {
      userId: this.getCurrentUserId(),
      request,
    });
  }

  async updateProveedor(request: UpdateProveedorRequest): Promise<Proveedor> {
    return await invoke<Proveedor>("update_proveedor", {
      userId: this.getCurrentUserId(),
      request,
    });
  }

  async deleteProveedor(id: number): Promise<void> {
    return await invoke<void>("delete_proveedor", {
      userId: this.getCurrentUserId(),
      id,
    });
  }
}
