import { defineStore } from "pinia";
import { ref } from "vue";
import type { Proveedor, CreateProveedorRequest, UpdateProveedorRequest } from "../../domain/entities";
import { toErrorMessage } from "../../infrastructure/api/errorHandler";
import { proveedorRepository } from "../../infrastructure/di";
import { ProveedorUseCase } from "../../application/usecases";

export const useProveedoresStore = defineStore("proveedores", () => {
  const proveedorUseCase = new ProveedorUseCase(proveedorRepository);
  const proveedores = ref<Proveedor[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchProveedores() {
    loading.value = true;
    error.value = null;
    try {
      proveedores.value = await proveedorUseCase.getAllProveedores();
    } catch (e) {
      error.value = toErrorMessage(e);
    } finally {
      loading.value = false;
    }
  }

  async function createProveedor(request: CreateProveedorRequest): Promise<boolean> {
    error.value = null;
    try {
      const newProveedor = await proveedorUseCase.createProveedor(request);
      proveedores.value.push(newProveedor);
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  async function updateProveedor(request: UpdateProveedorRequest): Promise<boolean> {
    error.value = null;
    try {
      const updated = await proveedorUseCase.updateProveedor(request);
      const index = proveedores.value.findIndex((p) => p.id === request.id);
      if (index !== -1) {
        proveedores.value[index] = updated;
      }
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  async function deleteProveedor(id: number): Promise<boolean> {
    error.value = null;
    try {
      await proveedorUseCase.deleteProveedor(id);
      proveedores.value = proveedores.value.filter((p) => p.id !== id);
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  return {
    proveedores,
    loading,
    error,
    fetchProveedores,
    createProveedor,
    updateProveedor,
    deleteProveedor,
  };
});
