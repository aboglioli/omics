import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config.service';
import { ISubscription } from '../models';

export interface IGetAllResponse {
  subscriptions: ISubscription[];
}

@Injectable()
export class SubscriptionService {
  private baseUrl: string;

  constructor(private http: HttpClient, configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/subscriptions`;
  }

  public getAll(): Observable<IGetAllResponse> {
    return this.http.get<IGetAllResponse>(this.baseUrl);
  }

  public unsubscribe(): Observable<any> {
    return this.http.delete(this.baseUrl);
  }
}
