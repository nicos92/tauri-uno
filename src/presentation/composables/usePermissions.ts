import { computed } from "vue";
import { useAuthStore } from "../stores";
import { PERMISSIONS, type PermissionValue } from "../../domain/entities";

export function usePermissions() {
  const authStore = useAuthStore();

  const allPermissions = computed(() => PERMISSIONS);

  function can(permission: PermissionValue): boolean {
    return authStore.hasPermission(permission);
  }

  function canViewUsers(): boolean {
    return can(PERMISSIONS.VIEW_USERS);
  }

  function canCreateUser(): boolean {
    return can(PERMISSIONS.CREATE_USER);
  }

  function canUpdateUser(): boolean {
    return can(PERMISSIONS.UPDATE_USER);
  }

  function canDeleteUser(): boolean {
    return can(PERMISSIONS.DELETE_USER);
  }

  function canViewPermissions(): boolean {
    return can(PERMISSIONS.VIEW_PERMISSIONS);
  }

  function canAssignPermission(): boolean {
    return can(PERMISSIONS.ASSIGN_PERMISSION);
  }

  function canRemovePermission(): boolean {
    return can(PERMISSIONS.REMOVE_PERMISSION);
  }

  function canViewProveedores(): boolean {
    return can(PERMISSIONS.VIEW_PROVEEDORES);
  }

  function canCreateProveedor(): boolean {
    return can(PERMISSIONS.CREATE_PROVEEDOR);
  }

  function canUpdateProveedor(): boolean {
    return can(PERMISSIONS.UPDATE_PROVEEDOR);
  }

  function canDeleteProveedor(): boolean {
    return can(PERMISSIONS.DELETE_PROVEEDOR);
  }

  function canViewCategorias(): boolean {
    return can(PERMISSIONS.VIEW_CATEGORIAS);
  }

  function canCreateCategoria(): boolean {
    return can(PERMISSIONS.CREATE_CATEGORIA);
  }

  function canUpdateCategoria(): boolean {
    return can(PERMISSIONS.UPDATE_CATEGORIA);
  }

  function canDeleteCategoria(): boolean {
    return can(PERMISSIONS.DELETE_CATEGORIA);
  }

  function canViewSubCategorias(): boolean {
    return can(PERMISSIONS.VIEW_SUB_CATEGORIAS);
  }

  function canCreateSubCategoria(): boolean {
    return can(PERMISSIONS.CREATE_SUB_CATEGORIA);
  }

  function canUpdateSubCategoria(): boolean {
    return can(PERMISSIONS.UPDATE_SUB_CATEGORIA);
  }

  function canDeleteSubCategoria(): boolean {
    return can(PERMISSIONS.DELETE_SUB_CATEGORIA);
  }

  function canViewArticulos(): boolean {
    return can(PERMISSIONS.VIEW_ARTICULOS);
  }

  function canCreateArticulo(): boolean {
    return can(PERMISSIONS.CREATE_ARTICULO);
  }

  function canUpdateArticulo(): boolean {
    return can(PERMISSIONS.UPDATE_ARTICULO);
  }

  function canDeleteArticulo(): boolean {
    return can(PERMISSIONS.DELETE_ARTICULO);
  }

  function canViewStock(): boolean {
    return can(PERMISSIONS.VIEW_STOCK);
  }

  function canCreateStock(): boolean {
    return can(PERMISSIONS.CREATE_STOCK);
  }

  function canUpdateStock(): boolean {
    return can(PERMISSIONS.UPDATE_STOCK);
  }

  function canDeleteStock(): boolean {
    return can(PERMISSIONS.DELETE_STOCK);
  }

  return {
    allPermissions,
    can,
    canViewUsers,
    canCreateUser,
    canUpdateUser,
    canDeleteUser,
    canViewPermissions,
    canAssignPermission,
    canRemovePermission,
    canViewProveedores,
    canCreateProveedor,
    canUpdateProveedor,
    canDeleteProveedor,
    canViewCategorias,
    canCreateCategoria,
    canUpdateCategoria,
    canDeleteCategoria,
    canViewSubCategorias,
    canCreateSubCategoria,
    canUpdateSubCategoria,
    canDeleteSubCategoria,
    canViewArticulos,
    canCreateArticulo,
    canUpdateArticulo,
    canDeleteArticulo,
    canViewStock,
    canCreateStock,
    canUpdateStock,
    canDeleteStock,
  };
}
