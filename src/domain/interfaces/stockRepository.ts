import type {
  Stock,
  CreateStockRequest,
  UpdateStockRequest,
} from "../../domain/entities";

export interface IStockRepository {
  getAllStock(): Promise<Stock[]>;
  getStockById(id: number): Promise<Stock>;
  getStockByArticulo(idArticulo: number): Promise<Stock | null>;
  createStock(request: CreateStockRequest): Promise<Stock>;
  updateStock(request: UpdateStockRequest): Promise<Stock>;
  deleteStock(id: number): Promise<void>;
  getPrecioVenta(id: number): Promise<number>;
}
