import { defineStore } from "pinia";
import { ref } from "vue";
import type { Stock, CreateStockRequest, UpdateStockRequest } from "../../domain/entities";
import { toErrorMessage } from "../../infrastructure/api/errorHandler";
import { stockRepository } from "../../infrastructure/di";

export const useStockStore = defineStore("stock", () => {
  const stocks = ref<Stock[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchStock() {
    loading.value = true;
    error.value = null;
    try {
      stocks.value = await stockRepository.getAllStock();
    } catch (e) {
      error.value = toErrorMessage(e);
    } finally {
      loading.value = false;
    }
  }

  async function getStockByArticulo(idArticulo: number): Promise<Stock | null> {
    try {
      return await stockRepository.getStockByArticulo(idArticulo);
    } catch (e) {
      error.value = toErrorMessage(e);
      return null;
    }
  }

  async function createStock(request: CreateStockRequest): Promise<boolean> {
    error.value = null;
    try {
      const newStock = await stockRepository.createStock(request);
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
      const updated = await stockRepository.updateStock(request);
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
      await stockRepository.deleteStock(id);
      stocks.value = stocks.value.filter((s) => s.id !== id);
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  async function getPrecioVenta(id: number): Promise<number | null> {
    try {
      return await stockRepository.getPrecioVenta(id);
    } catch (e) {
      error.value = toErrorMessage(e);
      return null;
    }
  }

  return {
    stocks,
    loading,
    error,
    fetchStock,
    getStockByArticulo,
    createStock,
    updateStock,
    deleteStock,
    getPrecioVenta,
  };
});
