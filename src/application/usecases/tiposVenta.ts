import type {
  TipoVenta,
  CreateTipoVentaRequest,
  UpdateTipoVentaRequest,
} from "../../domain/entities";
import type { ITipoVentaRepository } from "../../domain/interfaces";

export class TipoVentaUseCase {
  constructor(private repository: ITipoVentaRepository) {}

  async getAllTiposVenta(): Promise<TipoVenta[]> {
    return await this.repository.getAllTiposVenta();
  }

  async createTipoVenta(request: CreateTipoVentaRequest): Promise<TipoVenta> {
    return await this.repository.createTipoVenta(request);
  }

  async updateTipoVenta(request: UpdateTipoVentaRequest): Promise<TipoVenta> {
    return await this.repository.updateTipoVenta(request);
  }

  async deleteTipoVenta(id: number): Promise<void> {
    return await this.repository.deleteTipoVenta(id);
  }
}
