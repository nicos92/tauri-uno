import { invoke } from "@tauri-apps/api/core";
import type { Categoria, CreateCategoriaRequest, UpdateCategoriaRequest } from "../../domain/entities";

export class CategoriaApiRepository {
  private getCurrentUserId(): number {
    const stored = localStorage.getItem("currentUser");
    if (stored) {
      const user = JSON.parse(stored);
      return user.id;
    }
    return 0;
  }

  async getAllCategorias(): Promise<Categoria[]> {
    return await invoke<Categoria[]>("get_all_categorias", {
      userId: this.getCurrentUserId(),
    });
  }

  async createCategoria(request: CreateCategoriaRequest): Promise<Categoria> {
    return await invoke<Categoria>("create_categoria", {
      userId: this.getCurrentUserId(),
      request,
    });
  }

  async updateCategoria(request: UpdateCategoriaRequest): Promise<Categoria> {
    return await invoke<Categoria>("update_categoria", {
      userId: this.getCurrentUserId(),
      request,
    });
  }

  async deleteCategoria(id: number): Promise<void> {
    return await invoke<void>("delete_categoria", {
      userId: this.getCurrentUserId(),
      id,
    });
  }
}
