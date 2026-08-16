import { invoke } from "@tauri-apps/api/core";
import type { AuditLogFilters, AuditLogPage } from "../../domain/entities";
import type { IAuditRepository } from "../../domain/interfaces";
import { getCurrentUserId } from "../utils/currentUser";

export class AuditApiRepository implements IAuditRepository {

  async getAuditLogs(filters: AuditLogFilters): Promise<AuditLogPage> {
    return await invoke<AuditLogPage>("get_audit_logs", {
      userId: getCurrentUserId(),
      request: filters,
    });
  }
}
