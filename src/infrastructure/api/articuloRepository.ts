import { invoke } from "@tauri-apps/api/core";
import type { Articulo, CreateArticuloRequest, UpdateArticuloRequest } from "../../domain/entities";

export class ArticuloApiRepository {
  private getCurrentUserId(): number {
    const stored = localStorage.getItem("currentUser");
    if (stored) {
      const user = JSON.parse(stored);
      return user.id;
    }
    return 0;
  }

  async getAllArticulos(): Promise<Articulo[]> {
    return await invoke<Articulo[]>("get_all_articulos", {
      userId: this.getCurrentUserId(),
    });
  }

  async createArticulo(request: CreateArticuloRequest): Promise<Articulo> {
    return await invoke<Articulo>("create_articulo", {
      userId: this.getCurrentUserId(),
      request,
    });
  }

  async updateArticulo(request: UpdateArticuloRequest): Promise<Articulo> {
    return await invoke<Articulo>("update_articulo", {
      userId: this.getCurrentUserId(),
      request,
    });
  }

  async deleteArticulo(id: number): Promise<void> {
    return await invoke<void>("delete_articulo", {
      userId: this.getCurrentUserId(),
      id,
    });
  }
}
