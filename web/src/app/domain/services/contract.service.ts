import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config.service';
import { IContract } from '../models';

export interface ISearchCommand {
  publication_id?: string;
  status?: string;
  date_from?: string;
  date_to?: string;
  offset?: number;
  limit?: number;
  order_by?: string; // 'most_viewed' 'most_liked' 'newest' 'best_reviews'
}

export interface ISearchResponse {
  contracts: IContract[];
}

@Injectable()
export class ContractService {
  private baseUrl: string;

  constructor(private http: HttpClient, configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/contracts`;
  }

  public search(cmd: ISearchCommand, include: string = ''): Observable<ISearchResponse> {
    let params = new HttpParams();

    if (cmd.publication_id) {
      params = params.append('publication_id', cmd.publication_id);
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

    return this.http.get<ISearchResponse>(this.baseUrl);
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
