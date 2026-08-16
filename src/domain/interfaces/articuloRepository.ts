import type {
  Articulo,
  CreateArticuloRequest,
  UpdateArticuloRequest,
} from "../../domain/entities";

export interface IArticuloRepository {
  getAllArticulos(): Promise<Articulo[]>;
  createArticulo(request: CreateArticuloRequest): Promise<Articulo>;
  updateArticulo(request: UpdateArticuloRequest): Promise<Articulo>;
  deleteArticulo(id: number): Promise<void>;
}
