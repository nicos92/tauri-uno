import { invoke } from "@tauri-apps/api/core";
import type { AuditLog, AuditLogFilters } from "../../domain/entities";

export class AuditApiRepository {
  private getCurrentUserId(): number {
    const stored = localStorage.getItem("currentUser");
    if (stored) {
      const user = JSON.parse(stored);
      return user.id;
    }
    return 0;
  }

  async getAuditLogs(filters: AuditLogFilters): Promise<AuditLog[]> {
    return await invoke<AuditLog[]>("get_audit_logs", {
      userId: this.getCurrentUserId(),
      request: filters,
    });
  }
}
