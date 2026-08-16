import { invoke } from "@tauri-apps/api/core";
import type { Articulo, CreateArticuloRequest, UpdateArticuloRequest } from "../../domain/entities";
import type { IArticuloRepository } from "../../domain/interfaces";
import { getCurrentUserId } from "../utils/currentUser";

export class ArticuloApiRepository implements IArticuloRepository {

  async getAllArticulos(): Promise<Articulo[]> {
    return await invoke<Articulo[]>("get_all_articulos", {
      userId: getCurrentUserId(),
    });
  }

  async createArticulo(request: CreateArticuloRequest): Promise<Articulo> {
    return await invoke<Articulo>("create_articulo", {
      userId: getCurrentUserId(),
      request,
    });
  }

  async updateArticulo(request: UpdateArticuloRequest): Promise<Articulo> {
    return await invoke<Articulo>("update_articulo", {
      userId: getCurrentUserId(),
      request,
    });
  }

  async deleteArticulo(id: number): Promise<void> {
    return await invoke<void>("delete_articulo", {
      userId: getCurrentUserId(),
      id,
    });
  }
}
