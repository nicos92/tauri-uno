import { createRouter, createWebHistory } from "vue-router";
import { useAuthStore } from "../stores";
import { PERMISSIONS } from "../../domain/entities";

const routes = [
  {
    path: "/login",
    name: "login",
    component: () => import("../pages/LoginPage.vue"),
    meta: { requiresAuth: false },
  },
  {
    path: "/",
    component: () => import("../layouts/MainLayout.vue"),
    meta: { requiresAuth: true },
    children: [
      {
        path: "",
        name: "home",
        component: () => import("../pages/HomePage.vue"),
      },
      {
        path: "users",
        name: "users",
        component: () => import("../pages/UsersPage.vue"),
        meta: { permission: PERMISSIONS.VIEW_USERS },
      },
      {
        path: "proveedores",
        name: "proveedores",
        component: () => import("../pages/ProveedoresPage.vue"),
        meta: { permission: PERMISSIONS.VIEW_PROVEEDORES },
      },
      {
        path: "categorias",
        name: "categorias",
        component: () => import("../pages/CategoriasPage.vue"),
        meta: { permission: PERMISSIONS.VIEW_CATEGORIAS },
      },
      {
        path: "permissions",
        name: "permissions",
        component: () => import("../pages/PermissionsPage.vue"),
        meta: { permission: PERMISSIONS.VIEW_PERMISSIONS },
      },
      {
        path: "settings",
        name: "settings",
        component: () => import("../pages/SettingsPage.vue"),
      },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

router.beforeEach((to, _from, next) => {
  const authStore = useAuthStore();
  authStore.loadFromStorage();

  if (to.meta.requiresAuth && !authStore.isAuthenticated) {
    next({ name: "login" });
  } else if (to.name === "login" && authStore.isAuthenticated) {
    next({ name: "home" });
  } else if (to.meta.permission && typeof to.meta.permission === "string") {
    if (authStore.hasPermission(to.meta.permission)) {
      next();
    } else {
      next({ name: "home" });
    }
  } else {
    next();
  }
});

export default router;
