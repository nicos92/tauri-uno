import { invoke } from "@tauri-apps/api/core";
import type { HomeStats } from "../../domain/entities";

export class HomeApiRepository {
  private getCurrentUserId(): number {
    const stored = sessionStorage.getItem("currentUser");
    if (stored) {
      const user = JSON.parse(stored);
      return user.id;
    }
    return 0;
  }

  async getHomeStats(): Promise<HomeStats> {
    return await invoke<HomeStats>("get_home_stats", {
      userId: this.getCurrentUserId(),
    });
  }
}
