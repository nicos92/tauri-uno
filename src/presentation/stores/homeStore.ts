import { defineStore } from "pinia";
import { ref } from "vue";
import type { HomeStats } from "../../domain/entities";
import { toErrorMessage } from "../../infrastructure/api/errorHandler";
import { HomeApiRepository } from "../../infrastructure/api/homeRepository";

const homeRepository = new HomeApiRepository();

export const useHomeStore = defineStore("home", () => {
  const stats = ref<HomeStats | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchStats() {
    loading.value = true;
    error.value = null;
    try {
      stats.value = await homeRepository.getHomeStats();
    } catch (e) {
      error.value = toErrorMessage(e);
    } finally {
      loading.value = false;
    }
  }

  return {
    stats,
    loading,
    error,
    fetchStats,
  };
});
