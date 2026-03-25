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
