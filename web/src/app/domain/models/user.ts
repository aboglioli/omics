export interface IRole {
  id: string;
  name: string;
}

export interface IUser {
  id: string;
  username: string;
  email?: string;
  name?: string;
  lastname?: string;
  birthdate?: string; // RFC 3339
  gender?: string; // male, female
  profile_image?: string; // allowed extensions: jpg, jpeg, png
  validated: boolean;
  role_id?: string;
  role?: IRole;
}
