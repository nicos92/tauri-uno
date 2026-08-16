<script setup lang="ts">
import { ref } from "vue";
import type { CartSourceItem } from "../../composables/useCart";
import { formatMoney } from "../../utils/format";

defineProps<{
  query: string;
  results: CartSourceItem[];
}>();

const emit = defineEmits<{
  "update:query": [value: string];
  select: [idArticulo: number];
  enter: [];
}>();

const inputEl = ref<HTMLInputElement | null>(null);

function focus() {
  inputEl.value?.focus();
}

defineExpose({ focus });
</script>

<template>
    <div class="form-group">
        <label>Buscar artículo por código o nombre</label>
        <input
            ref="inputEl"
            :value="query"
            type="text"
            placeholder="Escriba el código y presione Enter..."
            @input="
                emit('update:query', ($event.target as HTMLInputElement).value)
            "
            @keydown.enter.prevent="emit('enter')"
        />
    </div>
    <div v-if="results.length > 0" class="search-results">
        <button
            v-for="result in results"
            :key="result.id_articulo"
            type="button"
            @click="emit('select', result.id_articulo)"
            class="search-result-item"
        >
            <span class="result-code">{{ result.cod_articulo }}</span>
            <span class="result-name">{{ result.articulo }}</span>
            <span class="result-stock">
                Stock: {{ result.stockDisponible }}
            </span>
            <span class="result-precio">
                {{ formatMoney(result.precioVenta) }}
            </span>
        </button>
    </div>
    <div
        v-if="query.trim() && results.length === 0"
        class="empty-state small"
    >
        Sin coincidencias
    </div>
</template>

<style scoped>
.form-group {
    margin-bottom: 0.75rem;
}

.form-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
}

.form-group input {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid var(--color-border);
    border-radius: 6px;
    box-sizing: border-box;
    background: var(--color-surface);
    color: var(--color-text);
}

.search-results {
    border: 1px solid var(--color-border);
    border-radius: 8px;
    overflow: hidden;
}

.search-result-item {
    display: flex;
    align-items: center;
    gap: 1rem;
    width: 100%;
    padding: 0.75rem 1rem;
    background: var(--color-surface);
    border: none;
    border-bottom: 1px solid var(--color-border);
    cursor: pointer;
    text-align: left;
    color: var(--color-text);
}

.search-result-item:last-child {
    border-bottom: none;
}

.search-result-item:hover {
    background: var(--color-surface-2);
}

.result-code {
    font-weight: 600;
    min-width: 110px;
}

.result-name {
    flex: 1;
}

.result-stock {
    color: var(--color-text-muted);
    min-width: 90px;
}

.result-precio {
    font-weight: 600;
    min-width: 80px;
    text-align: right;
}

.empty-state {
    text-align: center;
    padding: 2rem;
    color: var(--color-text-muted);
}

.empty-state.small {
    padding: 1rem;
}
</style>
