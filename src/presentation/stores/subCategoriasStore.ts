import { defineStore } from "pinia";
import { ref } from "vue";
import type { SubCategoria, CreateSubCategoriaRequest, UpdateSubCategoriaRequest } from "../../domain/entities";
import { toErrorMessage } from "../../infrastructure/api/errorHandler";
import { SubCategoriaApiRepository } from "../../infrastructure/api/subCategoriaRepository";

const subCategoriaRepository = new SubCategoriaApiRepository();

export const useSubCategoriasStore = defineStore("subCategorias", () => {
  const subCategorias = ref<SubCategoria[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchSubCategorias() {
    loading.value = true;
    error.value = null;
    try {
      subCategorias.value = await subCategoriaRepository.getAllSubCategorias();
    } catch (e) {
      error.value = toErrorMessage(e);
    } finally {
      loading.value = false;
    }
  }

  async function createSubCategoria(request: CreateSubCategoriaRequest): Promise<boolean> {
    error.value = null;
    try {
      const newSubCategoria = await subCategoriaRepository.createSubCategoria(request);
      subCategorias.value.push(newSubCategoria);
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  async function updateSubCategoria(request: UpdateSubCategoriaRequest): Promise<boolean> {
    error.value = null;
    try {
      const updated = await subCategoriaRepository.updateSubCategoria(request);
      const index = subCategorias.value.findIndex((s) => s.id === request.id);
      if (index !== -1) {
        subCategorias.value[index] = updated;
      }
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  async function deleteSubCategoria(id: number): Promise<boolean> {
    error.value = null;
    try {
      await subCategoriaRepository.deleteSubCategoria(id);
      subCategorias.value = subCategorias.value.filter((s) => s.id !== id);
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  return {
    subCategorias,
    loading,
    error,
    fetchSubCategorias,
    createSubCategoria,
    updateSubCategoria,
    deleteSubCategoria,
  };
});
