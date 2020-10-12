import { IPayment } from './payment';

export interface ISubscriptionPlan {
  id: string;
  price: number;
  assigned_at: string;
}

export interface ISubscriptionStatus {
  status: string;
  changed_at: string;
}

export interface ISubscription {
  id: string;
  user_id?: string;
  user?: string;
  plan: ISubscriptionPlan;
  payments: IPayment[];
  status: ISubscriptionStatus;
}
