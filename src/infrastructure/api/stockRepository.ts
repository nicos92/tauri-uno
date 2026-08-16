import { invoke } from "@tauri-apps/api/core";
import type { Stock, CreateStockRequest, UpdateStockRequest } from "../../domain/entities";
import { getCurrentUserId } from "../utils/currentUser";

export class StockApiRepository {

  async getAllStock(): Promise<Stock[]> {
    return await invoke<Stock[]>("get_all_stock", {
      userId: getCurrentUserId(),
    });
  }

  async getStockById(id: number): Promise<Stock> {
    return await invoke<Stock>("get_stock_by_id", {
      userId: getCurrentUserId(),
      id,
    });
  }

  async getStockByArticulo(idArticulo: number): Promise<Stock | null> {
    return await invoke<Stock | null>("get_stock_by_articulo", {
      userId: getCurrentUserId(),
      idArticulo,
    });
  }

  async createStock(request: CreateStockRequest): Promise<Stock> {
    return await invoke<Stock>("create_stock", {
      userId: getCurrentUserId(),
      request,
    });
  }

  async updateStock(request: UpdateStockRequest): Promise<Stock> {
    return await invoke<Stock>("update_stock", {
      userId: getCurrentUserId(),
      request,
    });
  }

  async deleteStock(id: number): Promise<void> {
    return await invoke<void>("delete_stock", {
      userId: getCurrentUserId(),
      id,
    });
  }

  async getPrecioVenta(id: number): Promise<number> {
    return await invoke<number>("get_precio_venta", {
      userId: getCurrentUserId(),
      id,
    });
  }
}
