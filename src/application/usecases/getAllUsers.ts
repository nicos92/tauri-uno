import type { User } from "../../domain/entities";
import type { IUserRepository } from "../../domain/interfaces";

export class GetAllUsersUseCase {
  constructor(private repository: IUserRepository) {}

  async execute(): Promise<User[]> {
    return await this.repository.getAllUsers();
  }
}
