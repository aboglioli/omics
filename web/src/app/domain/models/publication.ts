import { IAuthor } from './author';
import { ICategory } from './category';
import { IStatistics } from './statistics';


export interface IImage {
  url: string;
}

export interface IPage {
  number: number;
  images: string[];
}

export interface IPublication {
  id: string;
  author_id?: string;
  author?: IAuthor;
  name: string;
  synopsis: string;
  category_id?: string;
  category?: ICategory;
  tags: string[];
  statistics?: IStatistics;
  status?: string;
  pages?: IPage[];
}
