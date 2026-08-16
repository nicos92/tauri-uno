import { invoke } from "@tauri-apps/api/core";
import type { Categoria, CreateCategoriaRequest, UpdateCategoriaRequest } from "../../domain/entities";
import type { ICategoriaRepository } from "../../domain/interfaces";
import { getCurrentUserId } from "../utils/currentUser";

export class CategoriaApiRepository implements ICategoriaRepository {

  async getAllCategorias(): Promise<Categoria[]> {
    return await invoke<Categoria[]>("get_all_categorias", {
      userId: getCurrentUserId(),
    });
  }

  async createCategoria(request: CreateCategoriaRequest): Promise<Categoria> {
    return await invoke<Categoria>("create_categoria", {
      userId: getCurrentUserId(),
      request,
    });
  }

  async updateCategoria(request: UpdateCategoriaRequest): Promise<Categoria> {
    return await invoke<Categoria>("update_categoria", {
      userId: getCurrentUserId(),
      request,
    });
  }

  async deleteCategoria(id: number): Promise<void> {
    return await invoke<void>("delete_categoria", {
      userId: getCurrentUserId(),
      id,
    });
  }
}
