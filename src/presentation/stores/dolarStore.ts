import { defineStore } from "pinia";
import { computed, ref } from "vue";
import type { DollarQuote } from "../../domain/entities";
import { toErrorMessage } from "../../infrastructure/api/errorHandler";
import { dollarRepository } from "../../infrastructure/di";

export const useDolarStore = defineStore("dolar", () => {
  const quotes = ref<DollarQuote[]>([]);
  const loading = ref(false);
  const updating = ref(false);
  const error = ref<string | null>(null);
  const lastUpdated = ref<string | null>(null);

  const latest = computed<DollarQuote | null>(() => quotes.value[0] ?? null);

  async function fetchQuotes() {
    loading.value = true;
    error.value = null;
    try {
      quotes.value = await dollarRepository.getQuotes();
      if (quotes.value.length > 0) {
        lastUpdated.value = new Date().toISOString();
      }
    } catch (e) {
      error.value = toErrorMessage(e);
    } finally {
      loading.value = false;
    }
  }

  async function fetchManual(): Promise<boolean> {
    updating.value = true;
    error.value = null;
    try {
      quotes.value = await dollarRepository.fetchManual();
      lastUpdated.value = new Date().toISOString();
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    } finally {
      updating.value = false;
    }
  }

  async function deleteQuote(id: number): Promise<boolean> {
    updating.value = true;
    error.value = null;
    try {
      quotes.value = await dollarRepository.deleteQuote(id);
      lastUpdated.value = new Date().toISOString();
      return true;
    } catch (e) {
      error.value = toErrorMessage(e);
      return false;
    } finally {
      updating.value = false;
    }
  }

  return {
    quotes,
    latest,
    loading,
    updating,
    error,
    lastUpdated,
    fetchQuotes,
    fetchManual,
    deleteQuote,
  };
});
