import { invoke } from "@tauri-apps/api/core";
import type {
  CambiarEstadoPresupuestoRequest,
  CreatePresupuestoRequest,
  PresupuestoEstado,
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
    estado?: PresupuestoEstado;
    fecha_desde?: string;
    fecha_hasta?: string;
    query?: string;
  }): Promise<PresupuestoPage> {
    return await invoke<PresupuestoPage>("get_all_presupuestos", {
      userId: this.getCurrentUserId(),
      request: {
        limit: filters.limit,
        offset: filters.offset,
        estado: filters.estado,
        fecha_desde: filters.fecha_desde,
        fecha_hasta: filters.fecha_hasta,
        query: filters.query,
      },
    });
  }

  async cambiarEstadoPresupuesto(
    id: number,
    estado: PresupuestoEstado,
  ): Promise<void> {
    const request: CambiarEstadoPresupuestoRequest = { id, estado };
    await invoke<void>("cambiar_estado_presupuesto", {
      userId: this.getCurrentUserId(),
      request,
    });
  }
}
