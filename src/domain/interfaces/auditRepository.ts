import type { AuditLogFilters, AuditLogPage } from "../../domain/entities";

export interface IAuditRepository {
  getAuditLogs(filters: AuditLogFilters): Promise<AuditLogPage>;
}
