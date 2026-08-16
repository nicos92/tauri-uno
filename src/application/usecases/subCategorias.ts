import type {
  SubCategoria,
  CreateSubCategoriaRequest,
  UpdateSubCategoriaRequest,
} from "../../domain/entities";
import type { ISubCategoriaRepository } from "../../domain/interfaces";

export class SubCategoriaUseCase {
  constructor(private repository: ISubCategoriaRepository) {}

  async getAllSubCategorias(): Promise<SubCategoria[]> {
    return await this.repository.getAllSubCategorias();
  }

  async createSubCategoria(
    request: CreateSubCategoriaRequest,
  ): Promise<SubCategoria> {
    return await this.repository.createSubCategoria(request);
  }

  async updateSubCategoria(
    request: UpdateSubCategoriaRequest,
  ): Promise<SubCategoria> {
    return await this.repository.updateSubCategoria(request);
  }

  async deleteSubCategoria(id: number): Promise<void> {
    return await this.repository.deleteSubCategoria(id);
  }
}
