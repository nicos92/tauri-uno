import { invoke } from "@tauri-apps/api/core";
import type { Stock, CreateStockRequest, UpdateStockRequest } from "../../domain/entities";

export class StockApiRepository {
  private getCurrentUserId(): number {
    const stored = localStorage.getItem("currentUser");
    if (stored) {
      const user = JSON.parse(stored);
      return user.id;
    }
    return 0;
  }

  async getAllStock(): Promise<Stock[]> {
    return await invoke<Stock[]>("get_all_stock", {
      userId: this.getCurrentUserId(),
    });
  }

  async getStockById(id: number): Promise<Stock> {
    return await invoke<Stock>("get_stock_by_id", {
      userId: this.getCurrentUserId(),
      id,
    });
  }

  async getStockByArticulo(idArticulo: number): Promise<Stock | null> {
    return await invoke<Stock | null>("get_stock_by_articulo", {
      userId: this.getCurrentUserId(),
      idArticulo,
    });
  }

  async createStock(request: CreateStockRequest): Promise<Stock> {
    return await invoke<Stock>("create_stock", {
      userId: this.getCurrentUserId(),
      request,
    });
  }

  async updateStock(request: UpdateStockRequest): Promise<Stock> {
    return await invoke<Stock>("update_stock", {
      userId: this.getCurrentUserId(),
      request,
    });
  }

  async deleteStock(id: number): Promise<void> {
    return await invoke<void>("delete_stock", {
      userId: this.getCurrentUserId(),
      id,
    });
  }

  async getPrecioVenta(id: number): Promise<number> {
    return await invoke<number>("get_precio_venta", {
      userId: this.getCurrentUserId(),
      id,
    });
  }
}
