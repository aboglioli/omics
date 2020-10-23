import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config.service';
import { IPagination, ISubscription } from '../models';

export interface ISearchCommand {
  user_id?: string;
  plan_id?: string;
  status?: string;
  date_from?: string;
  date_to?: string;
  offset?: number;
  limit?: number;
  order_by?: string; // 'newest', 'oldest'
}

@Injectable()
export class SubscriptionService {
  private baseUrl: string;

  constructor(private http: HttpClient, configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/subscriptions`;
  }

  public search(cmd: ISearchCommand, include = ''): Observable<IPagination<ISubscription>> {
    let params = new HttpParams();

    if (cmd.user_id) {
      params = params.append('user_id', cmd.user_id);
    }

    if (cmd.plan_id) {
      params = params.append('plan_id', cmd.plan_id);
    }

    if (cmd.status) {
      params = params.append('status', cmd.status);
    }

    if (cmd.date_from) {
      params = params.append('date_from', cmd.date_from);
    }

    if (cmd.date_to) {
      params = params.append('date_to', cmd.date_to);
    }

    if (cmd.offset) {
      params = params.append('offset', cmd.offset.toString());
    }

    if (cmd.limit) {
      params = params.append('limit', cmd.limit.toString());
    }

    if (cmd.order_by) {
      params = params.append('order_by', cmd.order_by);
    }

    if (include) {
      params = params.append('include', include);
    }

    return this.http.get<IPagination<ISubscription>>(this.baseUrl, { params });
  }

  public unsubscribe(): Observable<any> {
    return this.http.delete(this.baseUrl);
  }
}
