import type {
  CreateVentaRequest,
  VentaPage,
  VentaWithDetalle,
} from "../../domain/entities";

export interface VentaQuery {
  limit: number;
  offset: number;
}

export interface IVentaRepository {
  getAllVentas(filters: VentaQuery): Promise<VentaPage>;
  getVentaById(id: number): Promise<VentaWithDetalle>;
  getVentasPorCliente(clienteId: number): Promise<VentaWithDetalle[]>;
  createVenta(request: CreateVentaRequest): Promise<VentaWithDetalle>;
  anularVenta(id: number): Promise<void>;
  isDiaCerrado(): Promise<boolean>;
}
