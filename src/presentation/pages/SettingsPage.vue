<script setup lang="ts">
import { ref, computed } from "vue";
import { useAuthStore } from "../stores";
import { useThemeStore } from "../stores/themeStore";

const authStore = useAuthStore();
const themeStore = useThemeStore();

const appVersion = ref("0.1.0");

const theme = computed({
    get: () => themeStore.mode,
    set: (value) => themeStore.setMode(value),
});

function handleLogout() {
    authStore.logout();
}
</script>

<template>
    <div class="settings-page">
        <h1>Configuración</h1>

        <div class="settings-section">
            <h3>Cuenta</h3>
            <div class="setting-item">
                <span class="setting-label">Usuario:</span>
                <span class="setting-value">{{
                    authStore.user?.username
                }}</span>
            </div>
            <div class="setting-item">
                <span class="setting-label">Estado:</span>
                <span class="setting-value">{{
                    authStore.user?.active ? "Activo" : "Inactivo"
                }}</span>
            </div>
        </div>

        <div class="settings-section">
            <h3>Aplicación</h3>
            <div class="setting-item">
                <span class="setting-label">Versión:</span>
                <span class="setting-value">{{ appVersion }}</span>
            </div>
            <div class="setting-item">
                <span class="setting-label">Tema:</span>
                <select v-model="theme" class="setting-select">
                    <option value="light">Claro</option>
                    <option value="dark">Oscuro</option>
                    <option value="system">Sistema</option>
                </select>
            </div>
        </div>

        <div class="settings-section">
            <h3>Sesión</h3>
            <button @click="handleLogout" class="btn-danger">
                Cerrar Sesión
            </button>
        </div>
    </div>
</template>

<style scoped>
.settings-page {
    padding: 2rem;
    max-width: 100%;
    background: var(--color-bg);
}

h1 {
    margin: 0 0 2rem;
}

.settings-section {
    background: var(--color-surface);
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    margin-bottom: 1.5rem;
}

.settings-section h3 {
    margin: 0 0 1rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--color-border);
}

.setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 0;
}

.setting-label {
    color: var(--color-text-muted);
}

.setting-value {
    font-weight: 500;
}

.setting-select {
    padding: 0.5rem 1rem;
    border: 1px solid var(--color-border);
    border-radius: 6px;
    background: var(--color-surface);
    color: var(--color-text);
}

.btn-danger {
    background: #e53e3e;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    cursor: pointer;
}

.btn-danger:hover {
    background: #c53030;
}
</style>
