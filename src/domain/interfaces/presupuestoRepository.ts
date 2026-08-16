import type {
  CreatePresupuestoRequest,
  PresupuestoEstado,
  PresupuestoPage,
  PresupuestoWithDetalle,
} from "../../domain/entities";

export interface PresupuestoQuery {
  limit: number;
  offset: number;
  estado?: PresupuestoEstado;
  fecha_desde?: string;
  fecha_hasta?: string;
  query?: string;
}

export interface IPresupuestoRepository {
  crearPresupuesto(
    request: CreatePresupuestoRequest,
  ): Promise<PresupuestoWithDetalle>;
  getPresupuestoById(id: number): Promise<PresupuestoWithDetalle>;
  getAllPresupuestos(filters: PresupuestoQuery): Promise<PresupuestoPage>;
  cambiarEstadoPresupuesto(
    id: number,
    estado: PresupuestoEstado,
  ): Promise<void>;
}
