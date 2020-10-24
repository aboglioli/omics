export interface ICount {
  [key: string]: number;
}

export interface IUsers {
  total: number;
  new: number;
  by_gender: ICount;
  by_age: ICount;
}

export interface IAuthors {
  total: number;
  new: number;
}

export interface IPublications {
  total: number;
  new: number;
  by_category: ICount;
  by_preferences: ICount;
}

export interface ISubscriptions {
  total: number;
  new: number;
  amount: number;
}

export interface IContracts {
  total: number;
  new: number;
  amount: number;
}

export interface IPayments {
  income: number;
  outcome: number;
}

export interface IReport {
  users?: IUsers;
  authors?: IAuthors;
  publications?: IPublications;
  subscriptions?: ISubscriptions;
  contracts?: IContracts;
  payments?: IPayments;

  from: string;
  to: string;
}
