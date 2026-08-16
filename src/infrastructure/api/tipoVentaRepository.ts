import { invoke } from "@tauri-apps/api/core";
import { getCurrentUserId } from "../utils/currentUser";
import type {
  CreateTipoVentaRequest,
  TipoVenta,
  UpdateTipoVentaRequest,
} from "../../domain/entities";
import type { ITipoVentaRepository } from "../../domain/interfaces";

export class TipoVentaApiRepository implements ITipoVentaRepository {

  async getAllTiposVenta(): Promise<TipoVenta[]> {
    return await invoke<TipoVenta[]>("get_all_tipos_venta", {
      userId: getCurrentUserId(),
    });
  }

  async createTipoVenta(request: CreateTipoVentaRequest): Promise<TipoVenta> {
    return await invoke<TipoVenta>("create_tipo_venta", {
      userId: getCurrentUserId(),
      request,
    });
  }

  async updateTipoVenta(request: UpdateTipoVentaRequest): Promise<TipoVenta> {
    return await invoke<TipoVenta>("update_tipo_venta", {
      userId: getCurrentUserId(),
      id: request.id,
      request: {
        nombre: request.nombre,
        hacia_donde: request.hacia_donde,
      },
    });
  }

  async deleteTipoVenta(id: number): Promise<void> {
    return await invoke<void>("delete_tipo_venta", {
      userId: getCurrentUserId(),
      id,
    });
  }
}
