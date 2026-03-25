import type { LoginRequest } from "../../domain/entities";
import type { IUserRepository, LoginResponse } from "../../domain/interfaces";

export class LoginUseCase {
  constructor(private repository: IUserRepository) {}

  async execute(username: string, password: string): Promise<LoginResponse> {
    const request: LoginRequest = { username, password };
    return await this.repository.login(request);
  }
}
