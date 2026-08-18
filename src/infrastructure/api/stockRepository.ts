import { invoke } from "@tauri-apps/api/core";
import type {
  Stock,
  CreateStockRequest,
  UpdateStockRequest,
  StockPreview,
  ApplyCostoPercentageRequest,
  ApplyCostoPercentageResult,
  CostUpdateOperationResponse,
  UndoOperationResult,
} from "../../domain/entities";
import type { IStockRepository } from "../../domain/interfaces";
import { getCurrentUserId } from "../utils/currentUser";

export class StockApiRepository implements IStockRepository {

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

  async getStockPreviewCosto(
    porcentaje: number,
    idCategoria: number | null,
    idSubCategoria: number | null,
    idProveedor: number | null,
  ): Promise<StockPreview[]> {
    return await invoke<StockPreview[]>("get_stock_preview_costo", {
      userId: getCurrentUserId(),
      porcentaje,
      idCategoria,
      idSubCategoria,
      idProveedor,
    });
  }

  async applyCostoPercentage(
    request: ApplyCostoPercentageRequest,
  ): Promise<ApplyCostoPercentageResult> {
    return await invoke<ApplyCostoPercentageResult>(
      "apply_costo_percentage_stock",
      { userId: getCurrentUserId(), request },
    );
  }

  async getLastUndoableCostUpdate(): Promise<CostUpdateOperationResponse | null> {
    return await invoke<CostUpdateOperationResponse | null>(
      "get_last_undoable_cost_update",
      { userId: getCurrentUserId() },
    );
  }

  async undoCostUpdate(operationId: number): Promise<UndoOperationResult> {
    return await invoke<UndoOperationResult>(
      "undo_cost_update",
      { userId: getCurrentUserId(), operationId },
    );
  }
}
