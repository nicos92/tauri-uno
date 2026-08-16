import type {
  CreatePresupuestoRequest,
  PresupuestoEstado,
  PresupuestoPage,
  PresupuestoWithDetalle,
} from "../../domain/entities";
import type {
  IPresupuestoRepository,
  PresupuestoQuery,
} from "../../domain/interfaces";

export class PresupuestoUseCase {
  constructor(private repository: IPresupuestoRepository) {}

  async crearPresupuesto(
    request: CreatePresupuestoRequest,
  ): Promise<PresupuestoWithDetalle> {
    return await this.repository.crearPresupuesto(request);
  }

  async getPresupuestoById(id: number): Promise<PresupuestoWithDetalle> {
    return await this.repository.getPresupuestoById(id);
  }

  async getAllPresupuestos(filters: PresupuestoQuery): Promise<PresupuestoPage> {
    return await this.repository.getAllPresupuestos(filters);
  }

  async cambiarEstadoPresupuesto(
    id: number,
    estado: PresupuestoEstado,
  ): Promise<void> {
    return await this.repository.cambiarEstadoPresupuesto(id, estado);
  }
}
