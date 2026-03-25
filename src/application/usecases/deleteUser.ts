import type { IUserRepository } from "../../domain/interfaces";

export class DeleteUserUseCase {
  constructor(private repository: IUserRepository) {}

  async execute(id: number): Promise<void> {
    return await this.repository.deleteUser(id);
  }
}
