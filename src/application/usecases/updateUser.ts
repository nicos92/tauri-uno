import type { User, UpdateUserRequest } from "../../domain/entities";
import type { IUserRepository } from "../../domain/interfaces";

export class UpdateUserUseCase {
  constructor(private repository: IUserRepository) {}

  async execute(id: number, username: string, active: boolean): Promise<User> {
    const request: UpdateUserRequest = { id, username, active };
    return await this.repository.updateUser(request);
  }
}
