<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import {
    useAuthStore,
    useArticulosStore,
    useUsersStore,
    useCategoriasStore,
    useSubCategoriasStore,
    useStockStore,
} from "../stores";

const authStore = useAuthStore();
const articulosStore = useArticulosStore();
const usersStore = useUsersStore();
const categoriasStore = useCategoriasStore();
const subCategoriasStore = useSubCategoriasStore();
const stockStore = useStockStore();

const loading = ref(true);

onMounted(async () => {
    try {
        await Promise.all([
            articulosStore.fetchArticulos(),
            usersStore.fetchUsers(),
            categoriasStore.fetchCategorias(),
            subCategoriasStore.fetchSubCategorias(),
            stockStore.fetchStock(),
        ]);
    } finally {
        loading.value = false;
    }
});

const totalArticulos = computed(() => articulosStore.articulos.length);
const totalUsuarios = computed(() => usersStore.users.length);
const totalCategorias = computed(() => categoriasStore.categorias.length);
const totalSubCategorias = computed(
    () => subCategoriasStore.subCategorias.length,
);

const categoriasConSubcategorias = computed(() => {
    return categoriasStore.categorias.map((cat) => {
        const subCats = subCategoriasStore.subCategorias.filter(
            (sc) => sc.id_categoria === cat.id,
        );
        return {
            ...cat,
            subCategorias: subCats,
        };
    });
});

const articulosBajoStock = computed(() => {
    return stockStore.stocks
        .filter((s) => s.cantidad < 100)
        .map((s) => {
            const articulo = articulosStore.articulos.find(
                (a) => a.id === s.id_articulo,
            );
            return {
                ...s,
                articuloNombre: articulo?.articulo || "Sin artículo",
                codArticulo: articulo?.cod_articulo || "-",
            };
        })
        .sort((a, b) => a.cantidad - b.cantidad);
});
</script>

<template>
    <div class="home-page">
        <h1>Bienvenido</h1>
        <p>
            Has iniciado sesión como:
            <strong>{{ authStore.user?.username }}</strong>
        </p>

        <div v-if="loading" class="loading">Cargando datos...</div>

        <template v-else>
            <div class="stats-cards">
                <div class="stat-card">
                    <div class="stat-icon blue">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        >
                            <path
                                d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"
                            />
                        </svg>
                    </div>
                    <div class="stat-info">
                        <span class="stat-value">{{ totalArticulos }}</span>
                        <span class="stat-label">Artículos</span>
                    </div>
                </div>

                <div class="stat-card">
                    <div class="stat-icon green">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        >
                            <path
                                d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"
                            />
                            <circle cx="9" cy="7" r="4" />
                            <path d="M23 21v-2a4 4 0 0 0-3-3.87" />
                            <path d="M16 3.13a4 4 0 0 1 0 7.75" />
                        </svg>
                    </div>
                    <div class="stat-info">
                        <span class="stat-value">{{ totalUsuarios }}</span>
                        <span class="stat-label">Usuarios</span>
                    </div>
                </div>

                <div class="stat-card">
                    <div class="stat-icon purple">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        >
                            <path
                                d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"
                            />
                        </svg>
                    </div>
                    <div class="stat-info">
                        <span class="stat-value">{{ totalCategorias }}</span>
                        <span class="stat-label">Categorías</span>
                    </div>
                </div>

                <div class="stat-card">
                    <div class="stat-icon orange">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        >
                            <path
                                d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"
                            />
                            <line x1="12" y1="11" x2="12" y2="17" />
                            <line x1="9" y1="14" x2="15" y2="14" />
                        </svg>
                    </div>
                    <div class="stat-info">
                        <span class="stat-value">{{ totalSubCategorias }}</span>
                        <span class="stat-label">Sub Categorías</span>
                    </div>
                </div>
            </div>

            <div class="card full-width">
                <h3>Stock Bajo</h3>
                <p class="card-description">
                    Artículos con menos de 10 unidades
                </p>
                <div class="table-container">
                    <table
                        v-if="articulosBajoStock.length > 0"
                        class="low-stock-table"
                    >
                        <thead>
                            <tr>
                                <th>Código</th>
                                <th>Artículo</th>
                                <th>Cantidad</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr
                                v-for="item in articulosBajoStock"
                                :key="item.id"
                            >
                                <td>{{ item.codArticulo }}</td>
                                <td>{{ item.articuloNombre }}</td>
                                <td
                                    class="quantity-cell"
                                    :class="{ critical: item.cantidad < 5 }"
                                >
                                    {{ item.cantidad }}
                                </td>
                            </tr>
                        </tbody>
                    </table>
                    <div v-else class="empty-state">
                        No hay artículos con stock bajo
                    </div>
                </div>
            </div>

            <div class="card full-width">
                <h3>Categorías y Sub Categorías</h3>
                <div class="categories-list">
                    <div
                        v-for="cat in categoriasConSubcategorias"
                        :key="cat.id"
                        class="category-item"
                    >
                        <div class="category-name">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="16"
                                height="16"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                            >
                                <path
                                    d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"
                                />
                            </svg>
                            {{ cat.categoria }}
                        </div>
                        <div class="subcategories">
                            <span
                                v-for="subCat in cat.subCategorias"
                                :key="subCat.id"
                                class="subcategory-tag"
                            >
                                {{ subCat.sub_categoria }}
                            </span>
                            <span
                                v-if="cat.subCategorias.length === 0"
                                class="no-subcategories"
                            >
                                Sin sub categorías
                            </span>
                        </div>
                    </div>
                    <div
                        v-if="categoriasConSubcategorias.length === 0"
                        class="empty-state"
                    >
                        No hay categorías registradas
                    </div>
                </div>
            </div>
        </template>
    </div>
</template>

<style scoped>
.home-page {
    padding: 2rem;
    background: var(--color-bg);
}

h1 {
    margin: 0 0 0.5rem;
    color: var(--color-text);
}

p {
    color: var(--color-text-muted);
    margin-bottom: 2rem;
}

.loading {
    text-align: center;
    padding: 2rem;
    color: var(--color-text-muted);
}

.stats-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1.5rem;
    margin-bottom: 2rem;
}

.stat-card {
    background: var(--color-surface);
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    display: flex;
    align-items: center;
    gap: 1rem;
}

.stat-icon {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
}

.stat-icon.blue {
    background: #e0e7ff;
    color: #4f46e5;
}

.stat-icon.green {
    background: #d1fae5;
    color: #059669;
}

.stat-icon.purple {
    background: #ede9fe;
    color: #7c3aed;
}

.stat-icon.orange {
    background: #ffedd5;
    color: #ea580c;
}

.stat-info {
    display: flex;
    flex-direction: column;
}

.stat-value {
    font-size: 1.75rem;
    font-weight: 700;
    color: var(--color-text);
}

.stat-label {
    font-size: 0.875rem;
    color: var(--color-text-muted);
}

.card {
    background: var(--color-surface);
    padding: 1.5rem;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.card.full-width {
    width: 100%;
    margin-bottom: 1.5rem;
}

.card h3 {
    margin: 0 0 0.5rem;
    color: #667eea;
}

.card p {
    margin: 0 0 1rem;
    color: var(--color-text-muted);
    font-size: 0.9rem;
}

.card-description {
    margin-top: -0.5rem;
    margin-bottom: 1rem;
}

.categories-list {
    max-height: 300px;
    overflow-y: auto;
}

.category-item {
    padding: 0.75rem 0;
    border-bottom: 1px solid var(--color-border);
}

.category-item:last-child {
    border-bottom: none;
}

.category-name {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-weight: 500;
    color: var(--color-text);
    margin-bottom: 0.5rem;
}

.subcategories {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    padding-left: 1.5rem;
}

.subcategory-tag {
    background: var(--color-surface-2);
    color: #667eea;
    padding: 0.25rem 0.75rem;
    border-radius: 20px;
    font-size: 0.8rem;
}

.no-subcategories {
    color: #999;
    font-size: 0.8rem;
    font-style: italic;
}

.table-container {
    max-height: 300px;
    overflow-y: auto;
}

.low-stock-table {
    width: 100%;
    border-collapse: collapse;
}

.low-stock-table th,
.low-stock-table td {
    padding: 0.75rem;
    text-align: left;
    border-bottom: 1px solid var(--color-border);
}

.low-stock-table th {
    font-weight: 600;
    color: var(--color-text-muted);
    font-size: 0.875rem;
    background: var(--color-surface-2);
}

.low-stock-table td {
    color: var(--color-text);
}

.quantity-cell {
    font-weight: 600;
    color: #f59e0b;
}

.quantity-cell.critical {
    color: #ef4444;
}

.config-card {
    margin-top: 0;
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

.empty-state {
    text-align: center;
    padding: 1.5rem;
    color: #999;
    font-style: italic;
}
</style>
