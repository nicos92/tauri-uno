import { invoke } from "@tauri-apps/api/core";
import { getCurrentUserId } from "../utils/currentUser";
import type {
  Cliente,
  CreateClienteRequest,
  UpdateClienteRequest,
} from "../../domain/entities";

export class ClienteApiRepository {

  async getAllClientes(): Promise<Cliente[]> {
    return await invoke<Cliente[]>("get_all_clientes", {
      userId: getCurrentUserId(),
    });
  }

  async getClienteById(id: number): Promise<Cliente> {
    return await invoke<Cliente>("get_cliente_by_id", {
      userId: getCurrentUserId(),
      id,
    });
  }

  async getClienteDefecto(): Promise<Cliente> {
    return await invoke<Cliente>("get_cliente_defecto", {
      userId: getCurrentUserId(),
    });
  }

  async crearCliente(request: CreateClienteRequest): Promise<Cliente> {
    return await invoke<Cliente>("crear_cliente", {
      userId: getCurrentUserId(),
      request,
    });
  }

  async actualizarCliente(request: UpdateClienteRequest): Promise<Cliente> {
    return await invoke<Cliente>("actualizar_cliente", {
      userId: getCurrentUserId(),
      request,
    });
  }

  async eliminarCliente(id: number): Promise<void> {
    return await invoke<void>("eliminar_cliente", {
      userId: getCurrentUserId(),
      id,
    });
  }
}
