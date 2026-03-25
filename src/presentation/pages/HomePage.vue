<script setup lang="ts">
import { useAuthStore } from "../stores";

const authStore = useAuthStore();
</script>

<template>
    <div class="home-page">
        <h1>Bienvenido</h1>
        <p>
            Has iniciado sesión como:
            <strong>{{ authStore.user?.username }}</strong>
        </p>

        <div class="dashboard-cards">
            <div class="card">
                <h3>Configuración</h3>
                <p>Ajustes generales del sistema</p>
                <router-link to="/settings" class="card-link"
                    >Ir a Configuración</router-link
                >
            </div>
        </div>
    </div>
</template>

<style scoped>
.home-page {
    padding: 2rem;
}

h1 {
    margin: 0 0 0.5rem;
    color: #333;
}

p {
    color: #666;
    margin-bottom: 2rem;
}

.dashboard-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1.5rem;
}

.card {
    background: white;
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.card h3 {
    margin: 0 0 0.5rem;
    color: #667eea;
}

.card p {
    margin: 0 0 1rem;
    color: #666;
    font-size: 0.9rem;
}

.card-link {
    display: inline-block;
    padding: 0.5rem 1rem;
    background: #667eea;
    color: white;
    text-decoration: none;
    border-radius: 6px;
    font-size: 0.9rem;
}

.card-link:hover {
    background: #5568d3;
}
</style>
