import { invoke } from "@tauri-apps/api/core";
import type { Proveedor, CreateProveedorRequest, UpdateProveedorRequest } from "../../domain/entities";
import { getCurrentUserId } from "../utils/currentUser";

export class ProveedorApiRepository {

  async getAllProveedores(): Promise<Proveedor[]> {
    return await invoke<Proveedor[]>("get_all_proveedores", {
      userId: getCurrentUserId(),
    });
  }

  async getProveedorById(id: number): Promise<Proveedor> {
    return await invoke<Proveedor>("get_proveedor_by_id", {
      userId: getCurrentUserId(),
      id,
    });
  }

  async createProveedor(request: CreateProveedorRequest): Promise<Proveedor> {
    return await invoke<Proveedor>("create_proveedor", {
      userId: getCurrentUserId(),
      request,
    });
  }

  async updateProveedor(request: UpdateProveedorRequest): Promise<Proveedor> {
    return await invoke<Proveedor>("update_proveedor", {
      userId: getCurrentUserId(),
      request,
    });
  }

  async deleteProveedor(id: number): Promise<void> {
    return await invoke<void>("delete_proveedor", {
      userId: getCurrentUserId(),
      id,
    });
  }
}
