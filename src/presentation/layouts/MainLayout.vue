<script setup lang="ts">
import { ref, computed } from "vue";
import { useRouter, useRoute } from "vue-router";
import { useAuthStore } from "../stores";

const router = useRouter();
const route = useRoute();
const authStore = useAuthStore();

const isSidebarOpen = ref(true);

const menuItems = computed(() => {
    const items = [
        { name: "home", label: "Inicio", icon: "home", permission: null },
        {
            name: "users",
            label: "Usuarios",
            icon: "users",
            permission: "ver_usuarios",
        },
        {
            name: "proveedores",
            label: "Proveedores",
            icon: "proveedor",
            permission: "ver_proveedor",
        },
        {
            name: "categorias",
            label: "Categorías",
            icon: "category",
            permission: "ver_categorias",
        },
        {
            name: "sub-categorias",
            label: "Sub Categorías",
            icon: "subcategory",
            permission: "ver_sub_categorias",
        },
        {
            name: "articulos",
            label: "Artículos",
            icon: "article",
            permission: "ver_articulos",
        },
        {
            name: "stock",
            label: "Stock",
            icon: "stock",
            permission: "ver_stock",
        },
        {
            name: "permissions",
            label: "Permisos",
            icon: "lock",
            permission: "ver_permisos",
        },
        {
            name: "settings",
            label: "Configuración",
            icon: "settigns",
            permission: null,
        },
    ];

    return items.filter((item) => {
        if (!item.permission) return true;
        return authStore.hasPermission(item.permission);
    });
});

function isActive(name: string): boolean {
    return route.name === name;
}

function handleLogout() {
    authStore.logout();
    router.push({ name: "login" });
}
</script>

<template>
    <div class="main-layout">
        <aside :class="['sidebar', { collapsed: !isSidebarOpen }]">
            <div class="sidebar-header">
                <h2 v-if="isSidebarOpen">
                    <img
                        src="/svg/thunderfill.svg"
                        alt="logo tienda"
                        class="nav-icon"
                    />Calise App
                </h2>
                <button
                    @click="isSidebarOpen = !isSidebarOpen"
                    class="toggle-btn"
                >
                    {{ isSidebarOpen ? "◀" : "▶" }}
                </button>
            </div>

            <nav class="sidebar-nav">
                <router-link
                    v-for="item in menuItems"
                    :key="item.name"
                    :to="{ name: item.name }"
                    :class="['nav-item', { active: isActive(item.name) }]"
                >
                    <img
                        :src="`/svg/${item.icon}.svg`"
                        :alt="item.label"
                        class="nav-icon"
                    />
                    <span v-if="isSidebarOpen" class="nav-label">{{
                        item.label
                    }}</span>
                </router-link>
            </nav>

            <div class="sidebar-footer">
                <div v-if="isSidebarOpen" class="user-info">
                    <span>{{ authStore.user?.username }}</span>
                </div>
                <button
                    @click="handleLogout"
                    class="logout-btn"
                    title="Cerrar Sesión"
                >
                    <img
                        src="/svg/logout.svg"
                        alt="Cerrar Sesión"
                        class="nav-icon"
                    />
                </button>
            </div>
        </aside>

        <main class="main-content">
            <router-view />
        </main>
    </div>
</template>

<style scoped>
.main-layout {
    display: flex;
    min-height: 100vh;
    background: #f8fafc;
}

.sidebar {
    width: 250px;
    background: #1e293b;
    color: white;
    display: flex;
    flex-direction: column;
    transition: width 0.3s ease;
}

.sidebar.collapsed {
    width: 60px;
}

.sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem;
    border-bottom: 1px solid #334155;
}

.sidebar-header h2 {
    margin: 0;
    font-size: 1.2rem;
    white-space: nowrap;
}

.toggle-btn {
    background: none;
    border: none;
    color: white;
    cursor: pointer;
    font-size: 1rem;
    padding: 0.25rem;
}

.sidebar-nav {
    flex: 1;
    padding: 1rem 0;
}

.nav-item {
    display: flex;
    align-items: center;
    padding: 0.75rem 1rem;
    color: #94a3b8;
    text-decoration: none;
    transition: all 0.2s;
}

.nav-item:hover {
    background: #334155;
    color: white;
}

.nav-item.active {
    background: #667e99;
    color: white;
}

.nav-icon {
    width: 20px;
    height: 20px;
}

.nav-label {
    margin-left: 0.5rem;
    white-space: nowrap;
}

.sidebar-footer {
    padding: 1rem;
    border-top: 1px solid #334155;
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.user-info {
    font-size: 0.9rem;
    color: #94a3b8;
}

.logout-btn {
    background: none;
    border: none;
    cursor: pointer;
    padding: 0.25rem;
}

.logout-btn img {
    width: 20px;
    height: 20px;
}

.main-content {
    flex: 1;
    overflow-y: auto;
}
</style>
