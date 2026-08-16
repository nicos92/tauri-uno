import { defineStore } from "pinia";
import { ref } from "vue";
import type { CierreWithTipos } from "../../domain/entities";
import { toErrorMessage } from "../../infrastructure/api/errorHandler";
import { cierreRepository } from "../../infrastructure/di";

export const useCierresStore = defineStore("cierres", () => {
  const cierres = ref<CierreWithTipos[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const total = ref(0);
  const limit = ref(10);
  const offset = ref(0);

  async function fetchCierres(filters?: { limit: number; offset: number }) {
    loading.value = true;
    error.value = null;
    try {
      if (filters) {
        limit.value = filters.limit;
        offset.value = filters.offset;
      }
      const page = await cierreRepository.getAllCierres({
        limit: limit.value,
        offset: offset.value,
      });
      cierres.value = page.items;
      total.value = page.total;
    } catch (e) {
      error.value = toErrorMessage(e);
    } finally {
      loading.value = false;
    }
  }

  async function crearCierre(fecha: string): Promise<boolean> {
    error.value = null;
    try {
      const cierre = await cierreRepository.crearCierre({ fecha });
      cierres.value.unshift(cierre);
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  async function reabrirCierre(fecha: string): Promise<boolean> {
    error.value = null;
    try {
      await cierreRepository.reabrirCierre(fecha);
      cierres.value = cierres.value.filter((c) => c.fecha !== fecha);
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  return {
    cierres,
    loading,
    error,
    total,
    limit,
    offset,
    fetchCierres,
    crearCierre,
    reabrirCierre,
  };
});
