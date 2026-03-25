import { computed } from "vue";
import { useAuthStore } from "../stores";
import { PERMISSIONS, type PermissionValue } from "../../domain/entities";

export function usePermissions() {
  const authStore = useAuthStore();

  const allPermissions = computed(() => PERMISSIONS);

  function can(permission: PermissionValue): boolean {
    return authStore.hasPermission(permission);
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

  function canAssignPermission(): boolean {
    return can(PERMISSIONS.ASSIGN_PERMISSION);
  }

  function canRemovePermission(): boolean {
    return can(PERMISSIONS.REMOVE_PERMISSION);
  }

  function canViewUsers(): boolean {
    return can(PERMISSIONS.VIEW_USERS);
  }

  function canViewPermissions(): boolean {
    return can(PERMISSIONS.VIEW_PERMISSIONS);
  }

  function canCreatePermission(): boolean {
    return can(PERMISSIONS.CREATE_PERMISSION);
  }

  return {
    allPermissions,
    can,
    canCreateUser,
    canUpdateUser,
    canDeleteUser,
    canAssignPermission,
    canRemovePermission,
    canViewUsers,
    canViewPermissions,
    canCreatePermission,
  };
}
