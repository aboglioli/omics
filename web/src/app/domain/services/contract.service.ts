import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config.service';
import { IContract } from '../models';

export interface IGetAllResponse {
  contracts: IContract[];
}


@Injectable()
export class ContractService {
  private baseUrl: string;

  constructor(private http: HttpClient, configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/contracts`;
  }

  public getAll(): Observable<IGetAllResponse> {
    return this.http.get<IGetAllResponse>(this.baseUrl);
  }

  public approve(id: string): Observable<any> {
    return this.http.post(`${this.baseUrl}/${id}/approve`, {});
  }

  public reject(id: string): Observable<any> {
    return this.http.post(`${this.baseUrl}/${id}/reject`, {});
  }

  public delete(id: string): Observable<any> {
    return this.http.delete(`${this.baseUrl}/${id}`);
  }

  public generateSatistics(): Observable<any> {
    return this.http.post(`${this.baseUrl}/statistics`, {});
  }

  public charge(id: string): Observable<any> {
    return this.http.post(`${this.baseUrl}/${id}/charge`, {});
  }
}
