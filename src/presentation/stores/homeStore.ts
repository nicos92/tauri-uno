import { defineStore } from "pinia";
import { ref } from "vue";
import type { HomeStats } from "../../domain/entities";
import { toErrorMessage } from "../../infrastructure/api/errorHandler";
import { homeRepository } from "../../infrastructure/di";
import { HomeUseCase } from "../../application/usecases";

export const useHomeStore = defineStore("home", () => {
  const homeUseCase = new HomeUseCase(homeRepository);
  const stats = ref<HomeStats | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchStats() {
    loading.value = true;
    error.value = null;
    try {
      stats.value = await homeUseCase.getHomeStats();
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
