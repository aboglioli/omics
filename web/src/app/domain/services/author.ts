import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config';
import { IAuthor } from '../models';

export interface ISearchCommand {
  name: string;
}

export interface ISearchResponse {
  authors: IAuthor[];
}

@Injectable()
export class AuthorService {
  private baseUrl: string;

  constructor(private http: HttpClient, configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/authors`;
  }

  public getById(id: string): Observable<IAuthor> {
    return this.http.get<IAuthor>(`${this.baseUrl}/${id}`);
  }

  public search(cmd: ISearchCommand): Observable<ISearchResponse> {
    let params = new HttpParams();
    params = params.append('name', cmd.name);
    return this.http.get<ISearchResponse>(`${this.baseUrl}`, { params });
  }
}
