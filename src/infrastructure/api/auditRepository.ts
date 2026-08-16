import { invoke } from "@tauri-apps/api/core";
import type { AuditLogFilters, AuditLogPage } from "../../domain/entities";
import { getCurrentUserId } from "../utils/currentUser";

export class AuditApiRepository {

  async getAuditLogs(filters: AuditLogFilters): Promise<AuditLogPage> {
    return await invoke<AuditLogPage>("get_audit_logs", {
      userId: getCurrentUserId(),
      request: filters,
    });
  }
}
