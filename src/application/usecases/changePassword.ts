import type { ChangePasswordRequest, User } from "../../domain/entities";
import type { IUserRepository } from "../../domain/interfaces";

export class ChangePasswordUseCase {
  constructor(private repository: IUserRepository) {}

  async execute(request: ChangePasswordRequest): Promise<User> {
    return await this.repository.changePassword(request);
  }
}
