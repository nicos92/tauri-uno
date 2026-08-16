import type {
  CierrePage,
  CierreWithTipos,
  CrearCierreRequest,
} from "../../domain/entities";

export interface CierreQuery {
  limit: number;
  offset: number;
}

export interface ICierreRepository {
  getAllCierres(filters: CierreQuery): Promise<CierrePage>;
  crearCierre(request: CrearCierreRequest): Promise<CierreWithTipos>;
  reabrirCierre(fecha: string): Promise<void>;
}
