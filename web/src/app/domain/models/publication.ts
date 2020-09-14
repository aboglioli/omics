import { IAuthor } from './author';
import { ICategory } from './category';
import { IStatistics } from './statistics';

export interface IImage {
  url: string;
}

export interface IPage {
  number: number;
  images: IImage;
}

export interface IStatus {
  status: string;
  changed_by?: string;
  comment?: string;
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
  cover: string;
  statistics?: IStatistics;
  status: IStatus;
  pages?: IPage[];
  created_at: string;
  updated_at?: string;
}
