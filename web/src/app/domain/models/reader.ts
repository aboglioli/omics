export interface IReader {
  id: string;
  username: string;
  name: string;
  lastname: string;
  subscribed: boolean;
}

export interface IReaderInteraction {
  viewed: boolean;
  read: boolean;
  liked: boolean;
  reviewed: boolean;
}
