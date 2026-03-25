export interface User {
  id: number;
  username: string;
  active: boolean;
  created_at: string;
  modified_at: string;
  permissions?: string[];
}

export interface Permission {
  id: number;
  permission: string;
  created: string;
}

export interface LoginRequest {
  username: string;
  password: string;
}

export interface CreateUserRequest {
  username: string;
  password: string;
}

export interface UpdateUserRequest {
  id: number;
  username: string;
  active: boolean;
}

export interface AddPermissionRequest {
  user_id: number;
  permission_id: number;
}
