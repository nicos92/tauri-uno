import { invoke } from "@tauri-apps/api/core";
import type { CierreWithTipos, CrearCierreRequest } from "../../domain/entities";

export class CierresApiRepository {
  private getCurrentUserId(): number {
    const stored = sessionStorage.getItem("currentUser");
    if (stored) {
      const user = JSON.parse(stored);
      return user.id;
    }
    return 0;
  }

  async getAllCierres(): Promise<CierreWithTipos[]> {
    return await invoke<CierreWithTipos[]>("get_all_cierres", {
      userId: this.getCurrentUserId(),
    });
  }

  async crearCierre(request: CrearCierreRequest): Promise<CierreWithTipos> {
    return await invoke<CierreWithTipos>("crear_cierre", {
      userId: this.getCurrentUserId(),
      request,
    });
  }

  async reabrirCierre(fecha: string): Promise<void> {
    return await invoke<void>("reabrir_cierre", {
      userId: this.getCurrentUserId(),
      request: { fecha },
    });
  }
}
