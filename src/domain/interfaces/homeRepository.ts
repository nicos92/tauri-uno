import type { HomeStats } from "../../domain/entities";

export interface IHomeRepository {
  getHomeStats(): Promise<HomeStats>;
}
