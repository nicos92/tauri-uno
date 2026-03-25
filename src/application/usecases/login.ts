import type { User, LoginRequest } from "../../domain/entities";
import type { IUserRepository } from "../../domain/interfaces";

export class LoginUseCase {
  constructor(private repository: IUserRepository) {}

  async execute(username: string, password: string): Promise<User> {
    const request: LoginRequest = { username, password };
    return await this.repository.login(request);
  }
}
