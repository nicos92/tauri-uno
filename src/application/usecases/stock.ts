import type {
  Stock,
  CreateStockRequest,
  UpdateStockRequest,
} from "../../domain/entities";
import type { IStockRepository } from "../../domain/interfaces";

export class StockUseCase {
  constructor(private repository: IStockRepository) {}

  async getAllStock(): Promise<Stock[]> {
    return await this.repository.getAllStock();
  }

  async getStockByArticulo(idArticulo: number): Promise<Stock | null> {
    return await this.repository.getStockByArticulo(idArticulo);
  }

  async createStock(request: CreateStockRequest): Promise<Stock> {
    return await this.repository.createStock(request);
  }

  async updateStock(request: UpdateStockRequest): Promise<Stock> {
    return await this.repository.updateStock(request);
  }

  async deleteStock(id: number): Promise<void> {
    return await this.repository.deleteStock(id);
  }

  async getPrecioVenta(id: number): Promise<number> {
    return await this.repository.getPrecioVenta(id);
  }
}
