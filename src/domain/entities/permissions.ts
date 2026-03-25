export const PERMISSIONS = {
  CREATE_USER: "crear_usuario",
  UPDATE_USER: "modificar_usuario",
  DELETE_USER: "eliminar_usuario",
  ASSIGN_PERMISSION: "asignar_permiso_a_usuario",
  REMOVE_PERMISSION: "quitar_permiso_a_usuario",
  VIEW_USERS: "ver_usuarios",
  VIEW_PERMISSIONS: "ver_permisos",
  CREATE_PERMISSION: "crear_permiso",
} as const;

export type PermissionKey = keyof typeof PERMISSIONS;
export type PermissionValue = (typeof PERMISSIONS)[PermissionKey];
