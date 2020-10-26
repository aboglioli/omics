import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config.service';
import { IDonation, IPagination } from '../models';

export interface ISearchCommand {
  author_id?: string;
  reader_id?: string;
  status?: string;
  date_from?: string;
  date_to?: string;
  offset?: number;
  limit?: number;
  order_by?: string; // 'newest' 'oldest', 'amount'
}

@Injectable()
export class DonationService {
  private baseUrl: string;

  constructor(private http: HttpClient, configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/donations`;
  }

  public search(cmd: ISearchCommand, include: string = ''): Observable<IPagination<IDonation>> {
    let params = new HttpParams();

    if (cmd.author_id) {
      params = params.append('author_id', cmd.author_id);
    }

    if (cmd.reader_id) {
      params = params.append('reader_id', cmd.reader_id);
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

    return this.http.get<IPagination<IDonation>>(this.baseUrl, { params });
  }

  public charge(): Observable<any> {
    return this.http.post(`${this.baseUrl}/charge`, {});
  }
}
