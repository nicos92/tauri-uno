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

  function canChangeUserPassword(): boolean {
    return can(PERMISSIONS.CHANGE_USER_PASSWORD);
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

  function canViewClientes(): boolean {
    return can(PERMISSIONS.VIEW_CLIENTES);
  }

  function canCreateCliente(): boolean {
    return can(PERMISSIONS.CREATE_CLIENTE);
  }

  function canUpdateCliente(): boolean {
    return can(PERMISSIONS.UPDATE_CLIENTE);
  }

  function canDeleteCliente(): boolean {
    return can(PERMISSIONS.DELETE_CLIENTE);
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

  function canViewVentas(): boolean {
    return can(PERMISSIONS.VIEW_VENTAS);
  }

  function canCreateVenta(): boolean {
    return can(PERMISSIONS.CREATE_VENTA);
  }

  function canAnularVenta(): boolean {
    return can(PERMISSIONS.ANULAR_VENTA);
  }

  function canVenderSinStock(): boolean {
    return can(PERMISSIONS.VENDER_SIN_STOCK);
  }

  function canGenerarPresupuesto(): boolean {
    return can(PERMISSIONS.GENERAR_PRESUPUESTO);
  }

  function canViewTiposVenta(): boolean {
    return can(PERMISSIONS.VIEW_TIPOS_VENTA);
  }

  function canCreateTipoVenta(): boolean {
    return can(PERMISSIONS.CREATE_TIPO_VENTA);
  }

  function canUpdateTipoVenta(): boolean {
    return can(PERMISSIONS.UPDATE_TIPO_VENTA);
  }

  function canDeleteTipoVenta(): boolean {
    return can(PERMISSIONS.DELETE_TIPO_VENTA);
  }

  function canViewAuditoria(): boolean {
    return can(PERMISSIONS.VIEW_AUDITORIA);
  }

  function canViewCierres(): boolean {
    return can(PERMISSIONS.VIEW_CIERRES);
  }

  function canCreateCierre(): boolean {
    return can(PERMISSIONS.CREATE_CIERRE);
  }

  function canReabrirCierre(): boolean {
    return can(PERMISSIONS.REABRIR_CIERRE);
  }

  return {
    allPermissions,
    can,
    canViewUsers,
    canCreateUser,
    canUpdateUser,
    canDeleteUser,
    canChangeUserPassword,
    canViewPermissions,
    canAssignPermission,
    canRemovePermission,
    canViewProveedores,
    canCreateProveedor,
    canUpdateProveedor,
    canDeleteProveedor,
    canViewClientes,
    canCreateCliente,
    canUpdateCliente,
    canDeleteCliente,
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
    canViewVentas,
    canCreateVenta,
    canAnularVenta,
    canVenderSinStock,
    canGenerarPresupuesto,
    canViewTiposVenta,
    canCreateTipoVenta,
    canUpdateTipoVenta,
    canDeleteTipoVenta,
    canViewAuditoria,
    canViewCierres,
    canCreateCierre,
    canReabrirCierre,
  };
}
