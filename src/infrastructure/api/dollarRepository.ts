import { invoke } from "@tauri-apps/api/core";
import type { DollarQuote } from "../../domain/entities";

export class DollarApiRepository {
  private getCurrentUserId(): number {
    const stored = sessionStorage.getItem("currentUser");
    if (stored) {
      const user = JSON.parse(stored);
      return user.id;
    }
    return 0;
  }

  async getQuotes(): Promise<DollarQuote[]> {
    return await invoke<DollarQuote[]>("get_dollar_quotes", {
      userId: this.getCurrentUserId(),
    });
  }

  async fetchManual(): Promise<DollarQuote[]> {
    return await invoke<DollarQuote[]>("fetch_dollar_rates_manual", {
      userId: this.getCurrentUserId(),
    });
  }

  async deleteQuote(id: number): Promise<DollarQuote[]> {
    return await invoke<DollarQuote[]>("delete_dollar_quote", {
      userId: this.getCurrentUserId(),
      id,
    });
  }
}
