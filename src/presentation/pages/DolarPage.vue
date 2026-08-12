<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useDolarStore } from "../stores";
import { useToasts } from "../composables/useToasts";
import { formatMoney } from "../utils/format";
import type { DollarRate } from "../../domain/entities";

const dolarStore = useDolarStore();
const { error: toastError, success: toastSuccess } = useToasts();

const INTERVAL_OPTIONS = [
  { label: "30 segundos", value: 30 },
  { label: "1 minuto", value: 60 },
  { label: "5 minutos", value: 300 },
  { label: "10 minutos", value: 600 },
  { label: "30 minutos", value: 1800 },
];

const selectedInterval = ref(300);

let unlistenUpdated: UnlistenFn | undefined;
let unlistenError: UnlistenFn | undefined;
let unsubscribe: (() => void) | undefined;

const oficial = ref<DollarRate | undefined>(undefined);
const blue = ref<DollarRate | undefined>(undefined);

async function handleRefresh() {
  const ok = await dolarStore.fetchManual();
  if (ok) {
    toastSuccess("Cotización actualizada.");
  } else {
    toastError(dolarStore.error || "No se pudo actualizar la cotización.");
  }
}

async function handleIntervalChange() {
  const ok = await dolarStore.setPollingInterval(selectedInterval.value);
  if (ok) {
    toastSuccess(`Actualización automática cada ${selectedInterval.value} segundos.`);
  } else {
    toastError(dolarStore.error || "No se pudo cambiar el intervalo.");
  }
}

onMounted(async () => {
  unsubscribe = dolarStore.$subscribe((_mutation, state) => {
    oficial.value = state.rates.find((r) => r.dollar_type === "oficial");
    blue.value = state.rates.find((r) => r.dollar_type === "blue");
  });

  unlistenUpdated = await listen<DollarRate[]>("dollar-rates-updated", (event) => {
    dolarStore.applyRates(event.payload);
  });

  unlistenError = await listen<string>("dollar-rates-fetch-error", () => {
    toastError("No se pudo actualizar la cotización del dólar.");
  });

  await dolarStore.fetchRates();
  oficial.value = dolarStore.rates.find((r) => r.dollar_type === "oficial");
  blue.value = dolarStore.rates.find((r) => r.dollar_type === "blue");
});

onUnmounted(() => {
  if (unlistenUpdated) unlistenUpdated();
  if (unlistenError) unlistenError();
  if (unsubscribe) unsubscribe();
});
</script>

<template>
    <div class="dolar-page">
        <div class="page-header">
            <h1>Cotización del Dólar</h1>
        </div>

        <div class="dolar-bar">
            <button
                class="btn-primary"
                :disabled="dolarStore.updating"
                @click="handleRefresh"
            >
                {{ dolarStore.updating ? "Actualizando..." : "Actualizar ahora" }}
            </button>
            <div class="interval-control">
                <label for="interval">Actualización automática</label>
                <select
                    id="interval"
                    v-model="selectedInterval"
                    class="filter-input"
                    @change="handleIntervalChange"
                >
                    <option
                        v-for="opt in INTERVAL_OPTIONS"
                        :key="opt.value"
                        :value="opt.value"
                    >
                        {{ opt.label }}
                    </option>
                </select>
            </div>
            <div class="toolbar-spacer"></div>
            <span v-if="dolarStore.lastUpdated" class="last-updated">
                Última actualización:
                {{ new Date(dolarStore.lastUpdated).toLocaleTimeString() }}
            </span>
        </div>

        <div v-if="dolarStore.loading" class="loading">Cargando...</div>

        <div v-if="dolarStore.error" class="error-banner">
            {{ dolarStore.error }}
        </div>

        <div class="cards">
            <div class="rate-card">
                <div class="rate-card-header">
                    <span class="rate-type">Dólar Oficial</span>
                    <span class="rate-date" v-if="oficial">
                        {{ oficial.updated_at }}
                    </span>
                </div>
                <div class="rate-values">
                    <div class="rate-value">
                        <span class="rate-label">Compra</span>
                        <span class="rate-amount">
                            {{ oficial ? formatMoney(oficial.buy_price) : "—" }}
                        </span>
                    </div>
                    <div class="rate-value">
                        <span class="rate-label">Venta</span>
                        <span class="rate-amount">
                            {{ oficial ? formatMoney(oficial.sell_price) : "—" }}
                        </span>
                    </div>
                </div>
            </div>

            <div class="rate-card">
                <div class="rate-card-header">
                    <span class="rate-type">Dólar Blue</span>
                    <span class="rate-date" v-if="blue">
                        {{ blue.updated_at }}
                    </span>
                </div>
                <div class="rate-values">
                    <div class="rate-value">
                        <span class="rate-label">Compra</span>
                        <span class="rate-amount">
                            {{ blue ? formatMoney(blue.buy_price) : "—" }}
                        </span>
                    </div>
                    <div class="rate-value">
                        <span class="rate-label">Venta</span>
                        <span class="rate-amount">
                            {{ blue ? formatMoney(blue.sell_price) : "—" }}
                        </span>
                    </div>
                </div>
            </div>
        </div>

        <div v-if="!dolarStore.loading && dolarStore.rates.length === 0" class="empty-state">
            No hay cotizaciones registradas.
        </div>
    </div>
</template>

<style scoped>
.dolar-page {
    padding: 2rem;
    background: var(--color-bg);
    min-height: 100%;
}

.page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
}

.page-header h1 {
    margin: 0;
}

.dolar-bar {
    display: flex;
    gap: 1rem;
    align-items: center;
    flex-wrap: wrap;
    margin-bottom: 1.5rem;
}

.toolbar-spacer {
    flex: 1;
}

.last-updated {
    color: var(--color-text-muted);
    font-size: 0.9rem;
}

.interval-control {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.interval-control label {
    font-size: 0.9rem;
    color: var(--color-text-muted);
}

.filter-input {
    padding: 0.6rem 0.75rem;
    border: 1px solid var(--color-border);
    border-radius: 6px;
    background: var(--color-surface);
    color: var(--color-text);
    font-size: 0.95rem;
}

select.filter-input {
    background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='12' height='8' viewBox='0 0 12 8'><path d='M1 1l5 5 5-5' fill='none' stroke='%236b7280' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'/></svg>");
    background-repeat: no-repeat;
    background-position: right 0.75rem center;
    padding-right: 2.5rem;
}

.btn-primary {
    background: #3F2281;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    cursor: pointer;
}

.btn-primary:hover:not(:disabled) {
    background: #5568d3;
}

.btn-primary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

.cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 1.5rem;
    margin-bottom: 1.5rem;
}

.rate-card {
    background: var(--color-surface);
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    padding: 1.5rem;
}

.rate-card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
}

.rate-type {
    font-weight: 600;
    font-size: 1.05rem;
}

.rate-date {
    color: var(--color-text-muted);
    font-size: 0.85rem;
}

.rate-values {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
}

.rate-value {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

.rate-label {
    color: var(--color-text-muted);
    font-size: 0.85rem;
}

.rate-amount {
    font-size: 1.5rem;
    font-weight: 700;
}

.error-banner {
    color: #e53e3e;
    background: rgba(229, 62, 62, 0.1);
    border: 1px solid rgba(229, 62, 62, 0.3);
    padding: 0.75rem 1rem;
    border-radius: 6px;
    margin-bottom: 1rem;
}

.loading,
.empty-state {
    text-align: center;
    padding: 2rem;
    color: var(--color-text-muted);
}
</style>
