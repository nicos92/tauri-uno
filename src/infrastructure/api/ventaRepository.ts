import { invoke } from "@tauri-apps/api/core";
import { getCurrentUserId } from "../utils/currentUser";
import type {
  CreateVentaRequest,
  VentaPage,
  VentaWithDetalle,
} from "../../domain/entities";
import type { IVentaRepository, VentaQuery } from "../../domain/interfaces";

export class VentasApiRepository implements IVentaRepository {

  async getAllVentas(filters: VentaQuery): Promise<VentaPage> {
    return await invoke<VentaPage>("get_all_ventas", {
      userId: getCurrentUserId(),
      request: { limit: filters.limit, offset: filters.offset },
    });
  }

  async getVentaById(id: number): Promise<VentaWithDetalle> {
    return await invoke<VentaWithDetalle>("get_venta_by_id", {
      userId: getCurrentUserId(),
      id,
    });
  }

  async getVentasPorCliente(clienteId: number): Promise<VentaWithDetalle[]> {
    return await invoke<VentaWithDetalle[]>("get_ventas_por_cliente", {
      userId: getCurrentUserId(),
      clienteId,
    });
  }

  async createVenta(request: CreateVentaRequest): Promise<VentaWithDetalle> {
    return await invoke<VentaWithDetalle>("create_venta", {
      userId: getCurrentUserId(),
      request,
    });
  }

  async anularVenta(id: number): Promise<void> {
    return await invoke<void>("anular_venta", {
      userId: getCurrentUserId(),
      id,
    });
  }

  async isDiaCerrado(): Promise<boolean> {
    return await invoke<boolean>("is_dia_cerrado", {
      userId: getCurrentUserId(),
    });
  }
}
