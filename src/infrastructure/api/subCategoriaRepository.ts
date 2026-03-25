import { invoke } from "@tauri-apps/api/core";
import type { SubCategoria, CreateSubCategoriaRequest, UpdateSubCategoriaRequest } from "../../domain/entities";

export class SubCategoriaApiRepository {
  private getCurrentUserId(): number {
    const stored = localStorage.getItem("currentUser");
    if (stored) {
      const user = JSON.parse(stored);
      return user.id;
    }
    return 0;
  }

  async getAllSubCategorias(): Promise<SubCategoria[]> {
    return await invoke<SubCategoria[]>("get_all_sub_categorias", {
      userId: this.getCurrentUserId(),
    });
  }

  async getSubCategoriasByCategoria(idCategoria: number): Promise<SubCategoria[]> {
    return await invoke<SubCategoria[]>("get_sub_categorias_by_categoria", {
      userId: this.getCurrentUserId(),
      idCategoria,
    });
  }

  async createSubCategoria(request: CreateSubCategoriaRequest): Promise<SubCategoria> {
    return await invoke<SubCategoria>("create_sub_categoria", {
      userId: this.getCurrentUserId(),
      request,
    });
  }

  async updateSubCategoria(request: UpdateSubCategoriaRequest): Promise<SubCategoria> {
    return await invoke<SubCategoria>("update_sub_categoria", {
      userId: this.getCurrentUserId(),
      request,
    });
  }

  async deleteSubCategoria(id: number): Promise<void> {
    return await invoke<void>("delete_sub_categoria", {
      userId: this.getCurrentUserId(),
      id,
    });
  }
}
