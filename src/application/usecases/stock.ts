import type {
  Stock,
  CreateStockRequest,
  UpdateStockRequest,
  StockPreview,
  ApplyCostoPercentageRequest,
  ApplyCostoPercentageResult,
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

  async getStockPreviewCosto(
    porcentaje: number,
    idCategoria: number | null,
    idSubCategoria: number | null,
    idProveedor: number | null,
  ): Promise<StockPreview[]> {
    return await this.repository.getStockPreviewCosto(
      porcentaje,
      idCategoria,
      idSubCategoria,
      idProveedor,
    );
  }

  async applyCostoPercentage(
    request: ApplyCostoPercentageRequest,
  ): Promise<ApplyCostoPercentageResult> {
    return await this.repository.applyCostoPercentage(request);
  }
}
