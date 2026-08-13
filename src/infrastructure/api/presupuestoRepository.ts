import { invoke } from "@tauri-apps/api/core";
import type {
  CreatePresupuestoRequest,
  PresupuestoPage,
  PresupuestoWithDetalle,
} from "../../domain/entities";

export class PresupuestoApiRepository {
  private getCurrentUserId(): number {
    const stored = sessionStorage.getItem("currentUser");
    if (stored) {
      const user = JSON.parse(stored);
      return user.id;
    }
    return 0;
  }

  async crearPresupuesto(
    request: CreatePresupuestoRequest,
  ): Promise<PresupuestoWithDetalle> {
    return await invoke<PresupuestoWithDetalle>("crear_presupuesto", {
      userId: this.getCurrentUserId(),
      request,
    });
  }

  async getPresupuestoById(id: number): Promise<PresupuestoWithDetalle> {
    return await invoke<PresupuestoWithDetalle>("get_presupuesto_by_id", {
      userId: this.getCurrentUserId(),
      id,
    });
  }

  async getAllPresupuestos(filters: {
    limit: number;
    offset: number;
  }): Promise<PresupuestoPage> {
    return await invoke<PresupuestoPage>("get_all_presupuestos", {
      userId: this.getCurrentUserId(),
      request: { limit: filters.limit, offset: filters.offset },
    });
  }
}
