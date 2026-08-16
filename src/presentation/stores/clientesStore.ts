import { defineStore } from "pinia";
import { ref } from "vue";
import type {
  Cliente,
  CreateClienteRequest,
  UpdateClienteRequest,
} from "../../domain/entities";
import { toErrorMessage } from "../../infrastructure/api/errorHandler";
import { clienteRepository } from "../../infrastructure/di";
import { ClienteUseCase } from "../../application/usecases";

export const useClientesStore = defineStore("clientes", () => {
  const clienteUseCase = new ClienteUseCase(clienteRepository);
  const clientes = ref<Cliente[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchClientes() {
    loading.value = true;
    error.value = null;
    try {
      clientes.value = await clienteUseCase.getAllClientes();
    } catch (e) {
      error.value = toErrorMessage(e);
    } finally {
      loading.value = false;
    }
  }

  async function getClienteDefecto(): Promise<Cliente | null> {
    error.value = null;
    try {
      return await clienteUseCase.getClienteDefecto();
    } catch (e) {
      error.value = toErrorMessage(e);
      return null;
    }
  }

  async function crearCliente(request: CreateClienteRequest): Promise<Cliente | null> {
    error.value = null;
    try {
      const nuevoCliente = await clienteUseCase.crearCliente(request);
      clientes.value.push(nuevoCliente);
      return nuevoCliente;
    } catch (e) {
      error.value = toErrorMessage(e);
      return null;
    }
  }

  async function actualizarCliente(
    request: UpdateClienteRequest,
  ): Promise<boolean> {
    error.value = null;
    try {
      const updated = await clienteUseCase.actualizarCliente(request);
      const index = clientes.value.findIndex((c) => c.id === request.id);
      if (index !== -1) {
        clientes.value[index] = updated;
      }
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  async function eliminarCliente(id: number): Promise<boolean> {
    error.value = null;
    try {
      await clienteUseCase.eliminarCliente(id);
      clientes.value = clientes.value.filter((c) => c.id !== id);
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    }
  }

  return {
    clientes,
    loading,
    error,
    fetchClientes,
    getClienteDefecto,
    crearCliente,
    actualizarCliente,
    eliminarCliente,
  };
});
