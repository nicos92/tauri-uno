import type {
  Cliente,
  CreateClienteRequest,
  UpdateClienteRequest,
} from "../../domain/entities";

export interface IClienteRepository {
  getAllClientes(): Promise<Cliente[]>;
  getClienteById(id: number): Promise<Cliente>;
  getClienteDefecto(): Promise<Cliente>;
  crearCliente(request: CreateClienteRequest): Promise<Cliente>;
  actualizarCliente(request: UpdateClienteRequest): Promise<Cliente>;
  eliminarCliente(id: number): Promise<void>;
}
