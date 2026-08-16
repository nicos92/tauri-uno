import { defineStore } from "pinia";
import { ref } from "vue";
import type { Articulo, CreateArticuloRequest, UpdateArticuloRequest } from "../../domain/entities";
import { toErrorMessage } from "../../infrastructure/api/errorHandler";
import { articuloRepository } from "../../infrastructure/di";

export const useArticulosStore = defineStore("articulos", () => {
  const articulos = ref<Articulo[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchArticulos() {
    loading.value = true;
    error.value = null;
    try {
      articulos.value = await articuloRepository.getAllArticulos();
    } catch (e) {
      error.value = toErrorMessage(e);
    } finally {
      loading.value = false;
    }
  }

  async function createArticulo(request: CreateArticuloRequest): Promise<boolean> {
    error.value = null;
    try {
      const newArticulo = await articuloRepository.createArticulo(request);
      articulos.value.push(newArticulo);
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  async function updateArticulo(request: UpdateArticuloRequest): Promise<boolean> {
    error.value = null;
    try {
      const updated = await articuloRepository.updateArticulo(request);
      const index = articulos.value.findIndex((a) => a.id === request.id);
      if (index !== -1) {
        articulos.value[index] = updated;
      }
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  async function deleteArticulo(id: number): Promise<boolean> {
    error.value = null;
    try {
      await articuloRepository.deleteArticulo(id);
      articulos.value = articulos.value.filter((a) => a.id !== id);
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  return {
    articulos,
    loading,
    error,
    fetchArticulos,
    createArticulo,
    updateArticulo,
    deleteArticulo,
  };
});
