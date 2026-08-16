import type {
  Categoria,
  CreateCategoriaRequest,
  UpdateCategoriaRequest,
} from "../../domain/entities";

export interface ICategoriaRepository {
  getAllCategorias(): Promise<Categoria[]>;
  createCategoria(request: CreateCategoriaRequest): Promise<Categoria>;
  updateCategoria(request: UpdateCategoriaRequest): Promise<Categoria>;
  deleteCategoria(id: number): Promise<void>;
}
