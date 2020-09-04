import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config.service';
import { IAuthor, IPublication, ICollection } from '../models';

export interface ISearchCommand {
  name?: string;
}

export interface ISearchResponse {
  authors: IAuthor[];
}

export interface IGetPublicationsResponse {
  publications: IPublication[];
}

export interface IGetCollectionsResponse {
  collections: ICollection[];
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

    if (cmd.name) {
      params = params.append('name', cmd.name);
    }

    return this.http.get<ISearchResponse>(`${this.baseUrl}`, { params });
  }

  public getPublications(id: string, include: string = ''): Observable<IGetPublicationsResponse> {
    let params = new HttpParams();

    if (include) {
      params = params.append('include', include);
    }

    return this.http.get<IGetPublicationsResponse>(`${this.baseUrl}/${id}`, { params });
  }

  public getCollections(id: string, include: string = ''): Observable<IGetCollectionsResponse> {
    let params = new HttpParams();

    if (include) {
      params = params.append('include', include);
    }

    return this.http.get<IGetCollectionsResponse>(`${this.baseUrl}/${id}`, { params });
  }

  public follow(id: string): Observable<any> {
    return this.http.post(`${this.baseUrl}/${id}/follow`, {});
  }
}
