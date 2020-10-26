import { IAuthor } from './author';
import { IReader } from './reader';
import { IPayment } from './payment';

export interface IDonationStatus {
  status: string;
  changed_at: string;
}

export interface IDonation {
  id: string;
  author_id?: IAuthor;
  author?: string;
  reader_id?: string;
  reader?: IReader;
  amount: number;
  comment: string;
  reader_payment?: IPayment;
  author_charge?: IPayment;
  status: IDonationStatus;
}
