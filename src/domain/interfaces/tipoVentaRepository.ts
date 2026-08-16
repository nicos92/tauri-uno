import type {
  CreateTipoVentaRequest,
  TipoVenta,
  UpdateTipoVentaRequest,
} from "../../domain/entities";

export interface ITipoVentaRepository {
  getAllTiposVenta(): Promise<TipoVenta[]>;
  createTipoVenta(request: CreateTipoVentaRequest): Promise<TipoVenta>;
  updateTipoVenta(request: UpdateTipoVentaRequest): Promise<TipoVenta>;
  deleteTipoVenta(id: number): Promise<void>;
}
