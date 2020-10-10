export interface IAuthor {
  id: string;
  username: string;
  name?: string;
  lastname?: string;
  biography?: string;
  profile_image?: string;
  followers: number;
  publications: number;
  created_at: string;
  updated_at?: string;
}
