import { defineStore } from "pinia";
import { ref } from "vue";
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
import { toErrorMessage } from "../../infrastructure/api/errorHandler";
import { stockRepository } from "../../infrastructure/di";
import { StockUseCase } from "../../application/usecases";

export const useStockStore = defineStore("stock", () => {
  const stockUseCase = new StockUseCase(stockRepository);
  const stocks = ref<Stock[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const lastOperation = ref<CostUpdateOperationResponse | null>(null);

  async function fetchStock() {
    loading.value = true;
    error.value = null;
    try {
      stocks.value = await stockUseCase.getAllStock();
    } catch (e) {
      error.value = toErrorMessage(e);
    } finally {
      loading.value = false;
    }
  }

  async function getStockByArticulo(idArticulo: number): Promise<Stock | null> {
    try {
      return await stockUseCase.getStockByArticulo(idArticulo);
    } catch (e) {
      error.value = toErrorMessage(e);
      return null;
    }
  }

  async function createStock(request: CreateStockRequest): Promise<boolean> {
    error.value = null;
    try {
      const newStock = await stockUseCase.createStock(request);
      stocks.value.push(newStock);
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  async function updateStock(request: UpdateStockRequest): Promise<boolean> {
    error.value = null;
    try {
      const updated = await stockUseCase.updateStock(request);
      const index = stocks.value.findIndex((s) => s.id === request.id);
      if (index !== -1) {
        stocks.value[index] = updated;
      }
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  async function deleteStock(id: number): Promise<boolean> {
    error.value = null;
    try {
      await stockUseCase.deleteStock(id);
      stocks.value = stocks.value.filter((s) => s.id !== id);
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  async function getPrecioVenta(id: number): Promise<number | null> {
    try {
      return await stockUseCase.getPrecioVenta(id);
    } catch (e) {
      error.value = toErrorMessage(e);
      return null;
    }
  }

  async function getStockPreviewCosto(
    porcentaje: number,
    idCategoria: number | null,
    idSubCategoria: number | null,
    idProveedor: number | null,
  ): Promise<StockPreview[]> {
    loading.value = true;
    error.value = null;
    try {
      return await stockUseCase.getStockPreviewCosto(
        porcentaje,
        idCategoria,
        idSubCategoria,
        idProveedor,
      );
    } catch (e) {
      error.value = toErrorMessage(e);
      return [];
    } finally {
      loading.value = false;
    }
  }

  async function applyCostoPercentage(
    request: ApplyCostoPercentageRequest,
  ): Promise<ApplyCostoPercentageResult | null> {
    error.value = null;
    try {
      const result = await stockUseCase.applyCostoPercentage(request);
      await fetchLastUndoable();
      return result;
    } catch (e) {
      error.value = toErrorMessage(e);
      return null;
    }
  }

  async function fetchLastUndoable() {
    try {
      lastOperation.value = await stockUseCase.getLastUndoableCostUpdate();
    } catch (e) {
      error.value = toErrorMessage(e);
    }
  }

  async function undoCostUpdate(): Promise<UndoOperationResult | null> {
    if (!lastOperation.value) return null;
    error.value = null;
    try {
      const result = await stockUseCase.undoCostUpdate(lastOperation.value.id);
      lastOperation.value = null;
      return result;
    } catch (e) {
      error.value = toErrorMessage(e);
      return null;
    }
  }

  return {
    stocks,
    loading,
    error,
    lastOperation,
    fetchStock,
    getStockByArticulo,
    createStock,
    updateStock,
    deleteStock,
    getPrecioVenta,
    getStockPreviewCosto,
    applyCostoPercentage,
    fetchLastUndoable,
    undoCostUpdate,
  };
});
