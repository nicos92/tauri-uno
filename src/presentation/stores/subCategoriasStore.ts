import { defineStore } from "pinia";
import { ref } from "vue";
import type { SubCategoria, CreateSubCategoriaRequest, UpdateSubCategoriaRequest } from "../../domain/entities";
import { toErrorMessage } from "../../infrastructure/api/errorHandler";
import { subCategoriaRepository } from "../../infrastructure/di";
import { SubCategoriaUseCase } from "../../application/usecases";

export const useSubCategoriasStore = defineStore("subCategorias", () => {
  const subCategoriaUseCase = new SubCategoriaUseCase(subCategoriaRepository);
  const subCategorias = ref<SubCategoria[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchSubCategorias() {
    loading.value = true;
    error.value = null;
    try {
      subCategorias.value = await subCategoriaUseCase.getAllSubCategorias();
    } catch (e) {
      error.value = toErrorMessage(e);
    } finally {
      loading.value = false;
    }
  }

  async function createSubCategoria(request: CreateSubCategoriaRequest): Promise<boolean> {
    error.value = null;
    try {
      const newSubCategoria = await subCategoriaUseCase.createSubCategoria(request);
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
      const updated = await subCategoriaUseCase.updateSubCategoria(request);
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
      await subCategoriaUseCase.deleteSubCategoria(id);
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
