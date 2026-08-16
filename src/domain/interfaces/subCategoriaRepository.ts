import type {
  SubCategoria,
  CreateSubCategoriaRequest,
  UpdateSubCategoriaRequest,
} from "../../domain/entities";

export interface ISubCategoriaRepository {
  getAllSubCategorias(): Promise<SubCategoria[]>;
  getSubCategoriasByCategoria(idCategoria: number): Promise<SubCategoria[]>;
  createSubCategoria(
    request: CreateSubCategoriaRequest,
  ): Promise<SubCategoria>;
  updateSubCategoria(
    request: UpdateSubCategoriaRequest,
  ): Promise<SubCategoria>;
  deleteSubCategoria(id: number): Promise<void>;
}
