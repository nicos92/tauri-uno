import type {
  Cliente,
  CreateClienteRequest,
  UpdateClienteRequest,
} from "../../domain/entities";
import type { IClienteRepository } from "../../domain/interfaces";

export class ClienteUseCase {
  constructor(private repository: IClienteRepository) {}

  async getAllClientes(): Promise<Cliente[]> {
    return await this.repository.getAllClientes();
  }

  async getClienteDefecto(): Promise<Cliente> {
    return await this.repository.getClienteDefecto();
  }

  async crearCliente(request: CreateClienteRequest): Promise<Cliente> {
    return await this.repository.crearCliente(request);
  }

  async actualizarCliente(request: UpdateClienteRequest): Promise<Cliente> {
    return await this.repository.actualizarCliente(request);
  }

  async eliminarCliente(id: number): Promise<void> {
    return await this.repository.eliminarCliente(id);
  }
}
