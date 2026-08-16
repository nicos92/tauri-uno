import { computed, type ComputedRef } from "vue";
import { useAuthStore } from "../stores";
import {
  PERMISSIONS,
  type PermissionKey,
  type PermissionValue,
} from "../../domain/entities";

type PascalCaseFromSnake<S extends string> =
  S extends `${infer A}_${infer B}`
    ? `${Capitalize<Lowercase<A>>}${PascalCaseFromSnake<B>}`
    : Capitalize<Lowercase<S>>;

type PermissionHelpers = {
  [K in PermissionKey as `can${PascalCaseFromSnake<K>}`]: () => boolean;
};

export type UsePermissionsResult = PermissionHelpers & {
  allPermissions: ComputedRef<typeof PERMISSIONS>;
  can: (permission: PermissionValue) => boolean;
};

function helperNameFromKey(key: PermissionKey): string {
  return key
    .split("_")
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1).toLowerCase())
    .join("");
}

export function usePermissions(): UsePermissionsResult {
  const authStore = useAuthStore();

  const allPermissions = computed(() => PERMISSIONS);

  function can(permission: PermissionValue): boolean {
    return authStore.hasPermission(permission);
  }

  const helpers: Record<string, () => boolean> = {};
  const entries = Object.entries(PERMISSIONS) as [
    PermissionKey,
    PermissionValue,
  ][];
  for (const [key, value] of entries) {
    helpers[`can${helperNameFromKey(key)}`] = () => can(value);
  }

  return {
    allPermissions,
    can,
    ...helpers,
  } as UsePermissionsResult;
}
