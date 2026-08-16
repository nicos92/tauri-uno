import type {
  Proveedor,
  CreateProveedorRequest,
  UpdateProveedorRequest,
} from "../../domain/entities";

export interface IProveedorRepository {
  getAllProveedores(): Promise<Proveedor[]>;
  getProveedorById(id: number): Promise<Proveedor>;
  createProveedor(request: CreateProveedorRequest): Promise<Proveedor>;
  updateProveedor(request: UpdateProveedorRequest): Promise<Proveedor>;
  deleteProveedor(id: number): Promise<void>;
}
