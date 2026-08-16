import type { HomeStats } from "../../domain/entities";
import type { IHomeRepository } from "../../domain/interfaces";

export class HomeUseCase {
  constructor(private repository: IHomeRepository) {}

  async getHomeStats(): Promise<HomeStats> {
    return await this.repository.getHomeStats();
  }
}
