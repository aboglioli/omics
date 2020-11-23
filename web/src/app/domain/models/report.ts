export interface ICount {
  [key: string]: number;
}

export interface IUsers {
  total: number;
  by_status: ICount;
  by_gender: ICount;
  by_age: ICount;
}

export interface IAuthors {
  total: number;
  new: number;
}

export interface IPublications {
  total: number;
  by_category: ICount;
  by_contract: ICount;
  by_status: ICount;
  by_pages: ICount;
}

export interface ISubscriptions {
  total: number;
  by_payment: ICount;
  by_status: ICount;
  by_amount: ICount;
  amount: number;
}

export interface IContracts {
  total: number;
  by_summary: ICount;
  by_payment: ICount;
  by_status: ICount;
  by_amount: ICount;
  amount: number;
}

export interface IDonations {
  total: number;
  by_status: ICount;
  by_amount: ICount;
  amount: number;
}

export interface IPayments {
  total_income: number;
  subscription_income: number;
  donation_income: number;
  total_outcome: number;
  contract_outcome: number;
  donation_outcome: number;
}

export interface IReport {
  users?: IUsers;
  authors?: IAuthors;
  publications?: IPublications;
  subscriptions?: ISubscriptions;
  contracts?: IContracts;
  donations?: IDonations;
  payments?: IPayments;

  from: string;
  to: string;
}
