import type {
  Stock,
  CreateStockRequest,
  UpdateStockRequest,
  StockPreview,
  ApplyCostoPercentageRequest,
  ApplyCostoPercentageResult,
} from "../../domain/entities";

export interface IStockRepository {
  getAllStock(): Promise<Stock[]>;
  getStockById(id: number): Promise<Stock>;
  getStockByArticulo(idArticulo: number): Promise<Stock | null>;
  createStock(request: CreateStockRequest): Promise<Stock>;
  updateStock(request: UpdateStockRequest): Promise<Stock>;
  deleteStock(id: number): Promise<void>;
  getPrecioVenta(id: number): Promise<number>;
  getStockPreviewCosto(
    porcentaje: number,
    idCategoria: number | null,
    idSubCategoria: number | null,
    idProveedor: number | null,
  ): Promise<StockPreview[]>;
  applyCostoPercentage(
    request: ApplyCostoPercentageRequest,
  ): Promise<ApplyCostoPercentageResult>;
}
