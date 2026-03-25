import { createRouter, createWebHistory } from "vue-router";
import { useAuthStore } from "../stores";

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
      },
      {
        path: "permissions",
        name: "permissions",
        component: () => import("../pages/PermissionsPage.vue"),
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
  
  if (to.meta.requiresAuth && !authStore.isAuthenticated) {
    next({ name: "login" });
  } else if (to.name === "login" && authStore.isAuthenticated) {
    next({ name: "home" });
  } else {
    next();
  }
});

export default router;
