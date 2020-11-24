import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config.service';
import { IPlan } from '../models';

export interface IGetAllResponse {
  plans: IPlan[];
}

export interface ISubscribeResponse {
  id: string;
  payment_link: string;
}

export interface ICreateCommand {
  id: string;
  name: string;
  description: string;
  price: number;
}

export interface IUpdateCommand {
  name: string;
  description: string;
  price: number;
}

@Injectable()
export class PlanService {
  private baseUrl: string;

  constructor(private http: HttpClient, configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/plans`;
  }

  public getAll(): Observable<IGetAllResponse> {
    return this.http.get<IGetAllResponse>(this.baseUrl);
  }

  public subscribe(id?: string): Observable<ISubscribeResponse> {
    id = id || 'basic';
    return this.http.post<ISubscribeResponse>(`${this.baseUrl}/${id}/subscribe`, {});
  }

  public create(cmd: ICreateCommand): Observable<any> {
    return this.http.post(`${this.baseUrl}`, cmd);
  }

  public update(id: string, cmd: IUpdateCommand): Observable<any> {
    return this.http.put(`${this.baseUrl}/${id}`, cmd);
  }

  public delete(id: string): Observable<any> {
    return this.http.delete(`${this.baseUrl}/${id}`);
  }
}
