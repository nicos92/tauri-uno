import type {
  Articulo,
  CreateArticuloRequest,
  UpdateArticuloRequest,
} from "../../domain/entities";
import type { IArticuloRepository } from "../../domain/interfaces";

export class ArticuloUseCase {
  constructor(private repository: IArticuloRepository) {}

  async getAllArticulos(): Promise<Articulo[]> {
    return await this.repository.getAllArticulos();
  }

  async createArticulo(request: CreateArticuloRequest): Promise<Articulo> {
    return await this.repository.createArticulo(request);
  }

  async updateArticulo(request: UpdateArticuloRequest): Promise<Articulo> {
    return await this.repository.updateArticulo(request);
  }

  async deleteArticulo(id: number): Promise<void> {
    return await this.repository.deleteArticulo(id);
  }
}
