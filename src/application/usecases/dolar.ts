import type { DollarQuote } from "../../domain/entities";
import type { IDollarRepository } from "../../domain/interfaces";

export class DollarUseCase {
  constructor(private repository: IDollarRepository) {}

  async getQuotes(): Promise<DollarQuote[]> {
    return await this.repository.getQuotes();
  }

  async fetchManual(): Promise<DollarQuote[]> {
    return await this.repository.fetchManual();
  }

  async deleteQuote(id: number): Promise<DollarQuote[]> {
    return await this.repository.deleteQuote(id);
  }
}
