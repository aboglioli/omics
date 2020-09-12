import { IReader } from './reader';

export interface IReview {
  reader_id?: string;
  reader?: IReader;
  publication_id: string;
  stars: number;
  comment: string;
  created_at: string;
}
