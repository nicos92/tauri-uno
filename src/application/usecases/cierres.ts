import type { CierrePage, CierreWithTipos, CrearCierreRequest } from "../../domain/entities";
import type { CierreQuery, ICierreRepository } from "../../domain/interfaces";

export class CierreUseCase {
  constructor(private repository: ICierreRepository) {}

  async getAllCierres(filters: CierreQuery): Promise<CierrePage> {
    return await this.repository.getAllCierres(filters);
  }

  async crearCierre(request: CrearCierreRequest): Promise<CierreWithTipos> {
    return await this.repository.crearCierre(request);
  }

  async reabrirCierre(fecha: string): Promise<void> {
    return await this.repository.reabrirCierre(fecha);
  }
}
