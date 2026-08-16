import type {
  CreateVentaRequest,
  VentaPage,
  VentaWithDetalle,
} from "../../domain/entities";
import type {
  IPresupuestoRepository,
  IVentaRepository,
  VentaQuery,
} from "../../domain/interfaces";

export class VentaUseCase {
  constructor(private repository: IVentaRepository) {}

  async getAllVentas(filters: VentaQuery): Promise<VentaPage> {
    return await this.repository.getAllVentas(filters);
  }

  async getVentaById(id: number): Promise<VentaWithDetalle> {
    return await this.repository.getVentaById(id);
  }

  async getVentasPorCliente(clienteId: number): Promise<VentaWithDetalle[]> {
    return await this.repository.getVentasPorCliente(clienteId);
  }

  async createVenta(request: CreateVentaRequest): Promise<VentaWithDetalle> {
    return await this.repository.createVenta(request);
  }

  async anularVenta(id: number): Promise<void> {
    return await this.repository.anularVenta(id);
  }

  async isDiaCerrado(): Promise<boolean> {
    return await this.repository.isDiaCerrado();
  }
}

export interface ConvertirPresupuestoEnVentaResult {
  venta: VentaWithDetalle;
  presupuestoId?: number;
  presupuestoConvertido: boolean;
}

export class ConvertirPresupuestoEnVenta {
  constructor(
    private ventasRepository: IVentaRepository,
    private presupuestosRepository: IPresupuestoRepository,
  ) {}

  async execute(
    request: CreateVentaRequest,
    presupuestoId?: number,
  ): Promise<ConvertirPresupuestoEnVentaResult> {
    const venta = await this.ventasRepository.createVenta(request);
    let presupuestoConvertido = false;
    if (presupuestoId) {
      try {
        await this.presupuestosRepository.cambiarEstadoPresupuesto(
          presupuestoId,
          "convertido",
        );
        presupuestoConvertido = true;
      } catch {
        presupuestoConvertido = false;
      }
    }
    return { venta, presupuestoId, presupuestoConvertido };
  }
}
