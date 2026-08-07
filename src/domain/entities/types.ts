export interface User {
  id: number;
  username: string;
  active: boolean;
  created_at: string;
  modified_at: string;
  permissions?: string[];
}

export interface Permission {
  id: number;
  permission: string;
  created: string;
}

export interface UserPermission {
  id: number;
  permission: string;
  created: string;
  assigned_at: string;
}

export interface Proveedor {
  id: number;
  cuit?: string;
  proveedor: string;
  nombre: string;
  tel?: string;
  email?: string;
  observacion?: string;
}

export interface Categoria {
  id: number;
  categoria: string;
}

export interface TipoVenta {
  id: number;
  nombre: string;
  hacia_donde: string | null;
  created_at: string;
}

export interface CreateTipoVentaRequest {
  nombre: string;
  hacia_donde?: string;
}

export interface UpdateTipoVentaRequest {
  id: number;
  nombre: string;
  hacia_donde?: string;
}

export interface LoginRequest {
  username: string;
  password: string;
}

export interface CreateUserRequest {
  username: string;
  password: string;
}

export interface UpdateUserRequest {
  id: number;
  username: string;
  active: boolean;
}

export interface AddPermissionRequest {
  user_id: number;
  permission_id: number;
}

export interface CreateProveedorRequest {
  proveedor: string;
  nombre: string;
  cuit?: string;
  tel?: string;
  email?: string;
  observacion?: string;
}

export interface UpdateProveedorRequest {
  id: number;
  proveedor: string;
  nombre: string;
  cuit?: string;
  tel?: string;
  email?: string;
  observacion?: string;
}

export interface CreateCategoriaRequest {
  categoria: string;
}

export interface UpdateCategoriaRequest {
  id: number;
  categoria: string;
}

export interface SubCategoria {
  id: number;
  sub_categoria: string;
  id_categoria: number;
}

export interface CreateSubCategoriaRequest {
  sub_categoria: string;
  id_categoria: number;
}

export interface UpdateSubCategoriaRequest {
  id: number;
  sub_categoria: string;
  id_categoria: number;
}

export interface Articulo {
  id: number;
  articulo: string;
  cod_articulo: string;
  id_sub_categoria: number;
  id_proveedor: number;
}

export interface CreateArticuloRequest {
  articulo: string;
  cod_articulo: string;
  id_sub_categoria: number;
  id_proveedor: number;
}

export interface UpdateArticuloRequest {
  id: number;
  articulo: string;
  cod_articulo: string;
  id_sub_categoria: number;
  id_proveedor: number;
}

export interface Stock {
  id: number;
  id_articulo: number;
  cantidad: number;
  costo: number;
  ganancia: number;
}

export interface CreateStockRequest {
  id_articulo: number;
  cantidad: number;
  costo: number;
  ganancia: number;
}

export interface UpdateStockRequest {
  id: number;
  cantidad: number;
  costo: number;
  ganancia: number;
}

export interface AuditLog {
  id: number;
  user_id: number;
  username: string;
  screen: string;
  action: string;
  detail: string | null;
  created_at: string;
}

export interface AuditLogFilters {
  user_id?: number;
  screen?: string;
  action?: string;
  from?: string;
  to?: string;
  limit?: number;
  offset?: number;
}

export interface Venta {
  id: number;
  user_id: number;
  fecha: string;
  total: number;
  descuento: number;
  anulada: boolean;
  observacion: string | null;
  created_at: string;
}

export interface VentaDetalle {
  id: number;
  id_venta: number;
  id_articulo: number;
  cantidad: number;
  costo_unitario: number;
  precio_unitario: number;
  subtotal: number;
}

export interface VentaDetalleConArticulo {
  id: number;
  id_articulo: number;
  cod_articulo: string;
  articulo: string;
  cantidad: number;
  costo_unitario: number;
  precio_unitario: number;
  subtotal: number;
}

export interface VentaWithDetalle {
  id: number;
  user_id: number;
  username: string;
  fecha: string;
  subtotal: number;
  descuento: number;
  total: number;
  anulada: boolean;
  observacion: string | null;
  tipo_venta: string | null;
  created_at: string;
  items: VentaDetalleConArticulo[];
}

export interface CreateVentaDetalleRequest {
  id_articulo: number;
  cantidad: number;
  precio_unitario?: number;
}

export interface CreateVentaRequest {
  items: CreateVentaDetalleRequest[];
  descuento?: number;
  observacion?: string;
  id_tipo_venta?: number;
}

export interface Cierre {
  id: number;
  fecha: string;
  dia: number;
  mes: number;
  anio: number;
  total_costo: number;
  total_ganancia: number;
  total_venta: number;
  created_at: string;
}

export interface CierreTipo {
  id_tipo_venta: number;
  tipo_venta: string;
  total: number;
}

export interface CierreWithTipos extends Cierre {
  tipos: CierreTipo[];
}

export interface CrearCierreRequest {
  fecha: string;
}
