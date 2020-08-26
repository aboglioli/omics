import { IPublication, ITag } from './publication';
import { ICategory } from './category';

export interface ICollection {
  id: string;
  autor_id?: string;
  author?: string;
  name: string;
  synopsis: string;
  category_id?: string;
  category?: ICategory;
  tags: ITag[];
  publication: IPublication[];
}
