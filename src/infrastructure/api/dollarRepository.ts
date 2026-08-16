import { invoke } from "@tauri-apps/api/core";
import type { DollarQuote } from "../../domain/entities";
import { getCurrentUserId } from "../utils/currentUser";

export class DollarApiRepository {

  async getQuotes(): Promise<DollarQuote[]> {
    return await invoke<DollarQuote[]>("get_dollar_quotes", {
      userId: getCurrentUserId(),
    });
  }

  async fetchManual(): Promise<DollarQuote[]> {
    return await invoke<DollarQuote[]>("fetch_dollar_rates_manual", {
      userId: getCurrentUserId(),
    });
  }

  async deleteQuote(id: number): Promise<DollarQuote[]> {
    return await invoke<DollarQuote[]>("delete_dollar_quote", {
      userId: getCurrentUserId(),
      id,
    });
  }
}
