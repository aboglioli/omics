import { IAuthor } from './author';
import { ICategory } from './category';

export interface ICollection {
  id: string;
  author_id?: string;
  author?: IAuthor;
  name: string;
  synopsis: string;
  category_id?: string;
  category?: ICategory;
  cover: string;
  tags: string[];
  publications: number;
  created_at: string;
  updated_at?: string;
}
