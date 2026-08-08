import { invoke } from "@tauri-apps/api/core";
import type { CreateVentaRequest, VentaWithDetalle } from "../../domain/entities";

export class VentasApiRepository {
  private getCurrentUserId(): number {
    const stored = sessionStorage.getItem("currentUser");
    if (stored) {
      const user = JSON.parse(stored);
      return user.id;
    }
    return 0;
  }

  async getAllVentas(): Promise<VentaWithDetalle[]> {
    return await invoke<VentaWithDetalle[]>("get_all_ventas", {
      userId: this.getCurrentUserId(),
    });
  }

  async getVentaById(id: number): Promise<VentaWithDetalle> {
    return await invoke<VentaWithDetalle>("get_venta_by_id", {
      userId: this.getCurrentUserId(),
      id,
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
