import { invoke } from "@tauri-apps/api/core";
import type {
  CreateVentaRequest,
  VentaPage,
  VentaWithDetalle,
} from "../../domain/entities";

export class VentasApiRepository {
  private getCurrentUserId(): number {
    const stored = sessionStorage.getItem("currentUser");
    if (stored) {
      const user = JSON.parse(stored);
      return user.id;
    }
    return 0;
  }

  async getAllVentas(filters: {
    limit: number;
    offset: number;
  }): Promise<VentaPage> {
    return await invoke<VentaPage>("get_all_ventas", {
      userId: this.getCurrentUserId(),
      request: { limit: filters.limit, offset: filters.offset },
    });
  }

  async getVentaById(id: number): Promise<VentaWithDetalle> {
    return await invoke<VentaWithDetalle>("get_venta_by_id", {
      userId: this.getCurrentUserId(),
      id,
    });
  }

  async getVentasPorCliente(clienteId: number): Promise<VentaWithDetalle[]> {
    return await invoke<VentaWithDetalle[]>("get_ventas_por_cliente", {
      userId: this.getCurrentUserId(),
      clienteId,
    });
  }

  async createVenta(request: CreateVentaRequest): Promise<VentaWithDetalle> {
    return await invoke<VentaWithDetalle>("create_venta", {
      userId: this.getCurrentUserId(),
      request,
    });
  }

  async anularVenta(id: number): Promise<void> {
    return await invoke<void>("anular_venta", {
      userId: this.getCurrentUserId(),
      id,
    });
  }

  async isDiaCerrado(): Promise<boolean> {
    return await invoke<boolean>("is_dia_cerrado", {
      userId: this.getCurrentUserId(),
    });
  }
}
