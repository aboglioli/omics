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

export interface ITag {
  id: string;
  name: string;
}

export interface IPublication {
  id: string;
  author_id?: string;
  author?: IAuthor;
  cover: string;
  name: string;
  collection: string;
  synopsis: string;
  category_id?: string;
  category?: ICategory;
  tags: ITag[];
  statistics?: IStatistics;
  status?: string;
  pages?: IPage[];
}
