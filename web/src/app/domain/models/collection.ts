import { ICategory } from './category';

export interface ICollection {
  id: string;
  autor_id?: string;
  author?: string;
  name: string;
  synopsis: string;
  category_id?: string;
  category?: ICategory;
  tags: string[];
  cover: string;
  publications: number;
  created_at: string;
  updated_at?: string;
}
