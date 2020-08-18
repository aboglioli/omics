import { IReader } from './reader';

export interface IReview {
  reader: IReader;
  publication_id: string;
  stars: number;
  comment: string;
}
