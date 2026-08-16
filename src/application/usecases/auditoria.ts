import type { AuditLogFilters, AuditLogPage } from "../../domain/entities";
import type { IAuditRepository } from "../../domain/interfaces";

export class AuditUseCase {
  constructor(private repository: IAuditRepository) {}

  async getAuditLogs(filters: AuditLogFilters): Promise<AuditLogPage> {
    return await this.repository.getAuditLogs(filters);
  }
}
