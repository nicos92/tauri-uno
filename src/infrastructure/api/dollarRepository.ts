import { invoke } from "@tauri-apps/api/core";
import type { DollarRate } from "../../domain/entities";

export class DollarApiRepository {
  private getCurrentUserId(): number {
    const stored = sessionStorage.getItem("currentUser");
    if (stored) {
      const user = JSON.parse(stored);
      return user.id;
    }
    return 0;
  }

  async getLatest(): Promise<DollarRate[]> {
    return await invoke<DollarRate[]>("get_latest_dollar_rates", {
      userId: this.getCurrentUserId(),
    });
  }

  async fetchManual(): Promise<DollarRate[]> {
    return await invoke<DollarRate[]>("fetch_dollar_rates_manual", {
      userId: this.getCurrentUserId(),
    });
  }

  async updatePollingInterval(seconds: number): Promise<number> {
    return await invoke<number>("update_polling_interval", {
      userId: this.getCurrentUserId(),
      seconds,
    });
  }
}
