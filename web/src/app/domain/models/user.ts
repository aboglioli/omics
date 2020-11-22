export const can = (role: IRole, permission: string): boolean =>
  role.permissions.some((p) => p.id === permission);

export interface IPermission {
  id: string;
  name: string;
}

export interface IRole {
  id: string;
  name: string;
  permissions: IPermission[];
  default: boolean;
  created_at: string;
  updated_at?: string;
}

export interface IUser {
  id: string;
  username: string;
  email: string;
  name?: string;
  lastname?: string;
  birthdate?: string; // RFC 3339
  gender?: string; // male, female
  biography?: string;
  profile_image?: string; // allowed extensions: jpg, jpeg, png
  validated: boolean;
  role_id?: string;
  role?: IRole;
  payment_email?: string;
  created_at: string;
  updated_at?: string;
}
