import { defineStore } from "pinia";
import { ref } from "vue";
import type { AuditLog, AuditLogFilters } from "../../domain/entities";
import { toErrorMessage } from "../../infrastructure/api/errorHandler";
import { auditRepository } from "../../infrastructure/di";
import { AuditUseCase } from "../../application/usecases";

export const useAuditStore = defineStore("audit", () => {
  const auditUseCase = new AuditUseCase(auditRepository);
  const logs = ref<AuditLog[]>([]);
  const total = ref(0);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchLogs(filters: AuditLogFilters): Promise<void> {
    loading.value = true;
    error.value = null;
    try {
      const page = await auditUseCase.getAuditLogs(filters);
      logs.value = page.items;
      total.value = page.total;
    } catch (e) {
      error.value = toErrorMessage(e);
    } finally {
      loading.value = false;
    }
  }

  return {
    logs,
    total,
    loading,
    error,
    fetchLogs,
  };
});
