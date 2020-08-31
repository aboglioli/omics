import { IAuthor } from './author';
import { ICategory } from './category';
import { IStatistics } from './statistics';

export interface IImage {
  url: string;
}

export interface IPage {
  number: number; // TODO: not used
  images: FileReader; // TODO: change to string?
}

export interface IPublication {
  id: string;
  author_id?: string;
  author?: IAuthor;
  cover: FileReader;
  name: string;
  collection: string;
  synopsis: string;
  category_id?: string;
  category?: ICategory;
  tags: string[];
  statistics?: IStatistics;
  status?: string;
  pages?: IPage[];
}
