import { defineStore } from "pinia";
import { ref } from "vue";
import type { Categoria, CreateCategoriaRequest, UpdateCategoriaRequest } from "../../domain/entities";
import { toErrorMessage } from "../../infrastructure/api/errorHandler";
import { categoriaRepository } from "../../infrastructure/di";

export const useCategoriasStore = defineStore("categorias", () => {
  const categorias = ref<Categoria[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchCategorias() {
    loading.value = true;
    error.value = null;
    try {
      categorias.value = await categoriaRepository.getAllCategorias();
    } catch (e) {
      error.value = toErrorMessage(e);
    } finally {
      loading.value = false;
    }
  }

  async function createCategoria(request: CreateCategoriaRequest): Promise<boolean> {
    error.value = null;
    try {
      const newCategoria = await categoriaRepository.createCategoria(request);
      categorias.value.push(newCategoria);
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  async function updateCategoria(request: UpdateCategoriaRequest): Promise<boolean> {
    error.value = null;
    try {
      const updated = await categoriaRepository.updateCategoria(request);
      const index = categorias.value.findIndex((c) => c.id === request.id);
      if (index !== -1) {
        categorias.value[index] = updated;
      }
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  async function deleteCategoria(id: number): Promise<boolean> {
    error.value = null;
    try {
      await categoriaRepository.deleteCategoria(id);
      categorias.value = categorias.value.filter((c) => c.id !== id);
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  return {
    categorias,
    loading,
    error,
    fetchCategorias,
    createCategoria,
    updateCategoria,
    deleteCategoria,
  };
});
