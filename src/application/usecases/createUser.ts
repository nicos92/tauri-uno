import type { User, CreateUserRequest } from "../../domain/entities";
import type { IUserRepository } from "../../domain/interfaces";

export class CreateUserUseCase {
  constructor(private repository: IUserRepository) {}

  async execute(username: string, password: string): Promise<User> {
    const request: CreateUserRequest = { username, password };
    return await this.repository.createUser(request);
  }
}
