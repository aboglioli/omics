import { ICategory } from './category';

export interface ICollection {
  id: string;
  author_id?: string;
  author?: string;
  name: string;
  created_at: Date;
  updated_at?: Date;
  synopsis: string;
  category_id?: string;
  category?: ICategory;
  cover: string;
  tags: string[];
}
