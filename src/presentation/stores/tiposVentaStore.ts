import { defineStore } from "pinia";
import { ref } from "vue";
import type {
  CreateTipoVentaRequest,
  TipoVenta,
  UpdateTipoVentaRequest,
} from "../../domain/entities";
import { toErrorMessage } from "../../infrastructure/api/errorHandler";
import { TipoVentaApiRepository } from "../../infrastructure/api/tipoVentaRepository";

const tipoVentaRepository = new TipoVentaApiRepository();

export const useTiposVentaStore = defineStore("tiposVenta", () => {
  const tipos = ref<TipoVenta[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchTiposVenta() {
    loading.value = true;
    error.value = null;
    try {
      tipos.value = await tipoVentaRepository.getAllTiposVenta();
    } catch (e) {
      error.value = toErrorMessage(e);
    } finally {
      loading.value = false;
    }
  }

  async function createTipoVenta(request: CreateTipoVentaRequest): Promise<boolean> {
    error.value = null;
    try {
      const newTipo = await tipoVentaRepository.createTipoVenta(request);
      tipos.value.push(newTipo);
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  async function updateTipoVenta(request: UpdateTipoVentaRequest): Promise<boolean> {
    error.value = null;
    try {
      const updated = await tipoVentaRepository.updateTipoVenta(request);
      const index = tipos.value.findIndex((t) => t.id === request.id);
      if (index !== -1) {
        tipos.value[index] = updated;
      }
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  async function deleteTipoVenta(id: number): Promise<boolean> {
    error.value = null;
    try {
      await tipoVentaRepository.deleteTipoVenta(id);
      tipos.value = tipos.value.filter((t) => t.id !== id);
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  return {
    tipos,
    loading,
    error,
    fetchTiposVenta,
    createTipoVenta,
    updateTipoVenta,
    deleteTipoVenta,
  };
});
