export const can = (user?: IUser, ...permissions: string[]): boolean => {
  if (!user || !user.role) {
    return false;
  }

  return user.role.permissions.some((rolePermission) =>
    permissions.every((permission) =>
      permission === rolePermission.id
    )
  );
};

export const canAny = (user?: IUser, ...permissions: string[]): boolean => {
  if (!user || !user.role) {
    return false;
  }

  return user.role.permissions.some((rolePermission) =>
    permissions.some((permission) =>
      permission === rolePermission.id
    )
  );
};

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
  flag?: number;
  created_at: string;
  updated_at?: string;
}
