import { invoke } from "@tauri-apps/api/core";
import type {
  Cliente,
  CreateClienteRequest,
  UpdateClienteRequest,
} from "../../domain/entities";

export class ClienteApiRepository {
  private getCurrentUserId(): number {
    const stored = sessionStorage.getItem("currentUser");
    if (stored) {
      const user = JSON.parse(stored);
      return user.id;
    }
    return 0;
  }

  async getAllClientes(): Promise<Cliente[]> {
    return await invoke<Cliente[]>("get_all_clientes", {
      userId: this.getCurrentUserId(),
    });
  }

  async getClienteById(id: number): Promise<Cliente> {
    return await invoke<Cliente>("get_cliente_by_id", {
      userId: this.getCurrentUserId(),
      id,
    });
  }

  async getClienteDefecto(): Promise<Cliente> {
    return await invoke<Cliente>("get_cliente_defecto", {
      userId: this.getCurrentUserId(),
    });
  }

  async crearCliente(request: CreateClienteRequest): Promise<Cliente> {
    return await invoke<Cliente>("crear_cliente", {
      userId: this.getCurrentUserId(),
      request,
    });
  }

  async actualizarCliente(request: UpdateClienteRequest): Promise<Cliente> {
    return await invoke<Cliente>("actualizar_cliente", {
      userId: this.getCurrentUserId(),
      request,
    });
  }

  async eliminarCliente(id: number): Promise<void> {
    return await invoke<void>("eliminar_cliente", {
      userId: this.getCurrentUserId(),
      id,
    });
  }
}
