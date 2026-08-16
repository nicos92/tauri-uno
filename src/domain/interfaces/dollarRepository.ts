import type { DollarQuote } from "../../domain/entities";

export interface IDollarRepository {
  getQuotes(): Promise<DollarQuote[]>;
  fetchManual(): Promise<DollarQuote[]>;
  deleteQuote(id: number): Promise<DollarQuote[]>;
}
