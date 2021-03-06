import { TypeAmount } from 'src/app/models/enums.model';

export interface IBusinessRules {
  days_to_generate_summaries: number;
  donation_percentage_retention: number;
  minimum_charge_amount: number;
  minimum_donation_amount: number;
  minimum_views_percentage_to_require_contract: number;
  subscription_percentage_retention: number;
}

export interface IBusinessRuleSingle {
  key: string;
  name: string;
  value: number;
  type: TypeAmount;
}

export interface IBusinessType {
  name: string;
  type: TypeAmount;
}
