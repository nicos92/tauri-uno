import type {
  Categoria,
  CreateCategoriaRequest,
  UpdateCategoriaRequest,
} from "../../domain/entities";
import type { ICategoriaRepository } from "../../domain/interfaces";

export class CategoriaUseCase {
  constructor(private repository: ICategoriaRepository) {}

  async getAllCategorias(): Promise<Categoria[]> {
    return await this.repository.getAllCategorias();
  }

  async createCategoria(request: CreateCategoriaRequest): Promise<Categoria> {
    return await this.repository.createCategoria(request);
  }

  async updateCategoria(request: UpdateCategoriaRequest): Promise<Categoria> {
    return await this.repository.updateCategoria(request);
  }

  async deleteCategoria(id: number): Promise<void> {
    return await this.repository.deleteCategoria(id);
  }
}
