import { invoke } from "@tauri-apps/api/core";
import type { AuditLogFilters, AuditLogPage } from "../../domain/entities";

export class AuditApiRepository {
  private getCurrentUserId(): number {
    const stored = sessionStorage.getItem("currentUser");
    if (stored) {
      const user = JSON.parse(stored);
      return user.id;
    }
    return 0;
  }

  async getAuditLogs(filters: AuditLogFilters): Promise<AuditLogPage> {
    return await invoke<AuditLogPage>("get_audit_logs", {
      userId: this.getCurrentUserId(),
      request: filters,
    });
  }
}
