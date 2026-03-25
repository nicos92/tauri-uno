export const PERMISSIONS = {
  VIEW_USERS: "ver_usuarios",
  CREATE_USER: "crear_usuario",
  UPDATE_USER: "modificar_usuario",
  DELETE_USER: "eliminar_usuario",
  VIEW_PERMISSIONS: "ver_permisos",
  ASSIGN_PERMISSION: "asignar_permiso_a_usuario",
  REMOVE_PERMISSION: "quitar_permiso_a_usuario",
  VIEW_PROVEEDORES: "ver_proveedor",
  CREATE_PROVEEDOR: "crear_proveedor",
  UPDATE_PROVEEDOR: "modificar_proveedor",
  DELETE_PROVEEDOR: "eliminar_proveedor",
  VIEW_CATEGORIAS: "ver_categorias",
  CREATE_CATEGORIA: "crear_categorias",
  UPDATE_CATEGORIA: "modificar_categorias",
  DELETE_CATEGORIA: "eliminar_categorias",
  VIEW_SUB_CATEGORIAS: "ver_sub_categorias",
  CREATE_SUB_CATEGORIA: "crear_sub_categorias",
  UPDATE_SUB_CATEGORIA: "modificar_sub_categorias",
  DELETE_SUB_CATEGORIA: "eliminar_sub_categorias",
  VIEW_ARTICULOS: "ver_articulos",
  CREATE_ARTICULO: "crear_articulos",
  UPDATE_ARTICULO: "modificar_articulos",
  DELETE_ARTICULO: "eliminar_articulos",
} as const;

export type PermissionKey = keyof typeof PERMISSIONS;
export type PermissionValue = (typeof PERMISSIONS)[PermissionKey];
