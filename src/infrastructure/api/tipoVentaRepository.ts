import { invoke } from "@tauri-apps/api/core";
import type {
  CreateTipoVentaRequest,
  TipoVenta,
  UpdateTipoVentaRequest,
} from "../../domain/entities";

export class TipoVentaApiRepository {
  private getCurrentUserId(): number {
    const stored = sessionStorage.getItem("currentUser");
    if (stored) {
      const user = JSON.parse(stored);
      return user.id;
    }
    return 0;
  }

  async getAllTiposVenta(): Promise<TipoVenta[]> {
    return await invoke<TipoVenta[]>("get_all_tipos_venta", {
      userId: this.getCurrentUserId(),
    });
  }

  async createTipoVenta(request: CreateTipoVentaRequest): Promise<TipoVenta> {
    return await invoke<TipoVenta>("create_tipo_venta", {
      userId: this.getCurrentUserId(),
      request,
    });
  }

  async updateTipoVenta(request: UpdateTipoVentaRequest): Promise<TipoVenta> {
    return await invoke<TipoVenta>("update_tipo_venta", {
      userId: this.getCurrentUserId(),
      id: request.id,
      request,
    });
  }

  async deleteTipoVenta(id: number): Promise<void> {
    return await invoke<void>("delete_tipo_venta", {
      userId: this.getCurrentUserId(),
      id,
    });
  }
}
