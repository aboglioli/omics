import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config.service';
import {
  IAuthor,
  ICollection,
  IPublication,
  IReader,
  ISubscription,
} from '../models';

export interface IGetFollowingResponse {
  authors: IAuthor[];
}

export interface IGetFavoritesResponse {
  publications: IPublication[];
  collections: ICollection[];
}

@Injectable()
export class ReaderService {
  private baseUrl: string;

  constructor(private http: HttpClient, configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/readers`;
  }

  public getById(id: string): Observable<IReader> {
    return this.http.get<IReader>(`${this.baseUrl}/${id}`);
  }

  public getFollowing(id: string): Observable<IGetFollowingResponse> {
    return this.http.get<IGetFollowingResponse>(`${this.baseUrl}/${id}/following`);
  }

  public getFavorites(id: string, include: string = ''): Observable<IGetFavoritesResponse> {
    let params = new HttpParams();

    if (include) {
      params = params.append('include', include);
    }

    return this.http.get<IGetFavoritesResponse>(`${this.baseUrl}/${id}/favorites`, { params });
  }

  public getSubscription(id: string): Observable<ISubscription> {
    return this.http.get<ISubscription>(`${this.baseUrl}/${id}/subscription`);
  }
}
