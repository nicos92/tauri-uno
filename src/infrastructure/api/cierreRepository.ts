import { invoke } from "@tauri-apps/api/core";
import { getCurrentUserId } from "../utils/currentUser";
import type {
  CierrePage,
  CierreWithTipos,
  CrearCierreRequest,
} from "../../domain/entities";

export class CierresApiRepository {

  async getAllCierres(filters: {
    limit: number;
    offset: number;
  }): Promise<CierrePage> {
    return await invoke<CierrePage>("get_all_cierres", {
      userId: getCurrentUserId(),
      request: { limit: filters.limit, offset: filters.offset },
    });
  }

  async crearCierre(request: CrearCierreRequest): Promise<CierreWithTipos> {
    return await invoke<CierreWithTipos>("crear_cierre", {
      userId: getCurrentUserId(),
      request,
    });
  }

  async reabrirCierre(fecha: string): Promise<void> {
    return await invoke<void>("reabrir_cierre", {
      userId: getCurrentUserId(),
      request: { fecha },
    });
  }
}
