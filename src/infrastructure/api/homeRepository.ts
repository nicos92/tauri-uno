import { invoke } from "@tauri-apps/api/core";
import type { HomeStats } from "../../domain/entities";
import type { IHomeRepository } from "../../domain/interfaces";
import { getCurrentUserId } from "../utils/currentUser";

export class HomeApiRepository implements IHomeRepository {

  async getHomeStats(): Promise<HomeStats> {
    return await invoke<HomeStats>("get_home_stats", {
      userId: getCurrentUserId(),
    });
  }
}
