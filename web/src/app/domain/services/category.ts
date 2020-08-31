import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config';
import { ICategory, IPublication, ICollection } from '../models';

export interface IGetAllResponse {
  categories: ICategory[];
}

export interface IGetPublicationsResponse {
  publications: IPublication[];
}

export interface IGetCollectionsResponse {
  collections: ICollection[];
}

@Injectable()
export class CategoryService {
  private baseUrl: string;

  constructor(private http: HttpClient, configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/categories`;
  }

  public getAll(): Observable<IGetAllResponse> {
    return this.http.get<IGetAllResponse>(`${this.baseUrl}`);
  }

  public getById(id: string): Observable<ICategory> {
    return this.http.get<ICategory>(`${this.baseUrl}/${id}`);
  }

  public getPublications(id: string, include: string = ''): Observable<IGetPublicationsResponse> {
    let params = new HttpParams();

    if (include) {
      params = params.append('include', include);
    }

    return this.http.get<IGetPublicationsResponse>(`${this.baseUrl}/${id}/publications`, { params });
  }

  public getCollections(id: string, include: string = ''): Observable<IGetCollectionsResponse> {
    let params = new HttpParams();

    if (include) {
      params = params.append('include', include);
    }

    return this.http.get<IGetCollectionsResponse>(`${this.baseUrl}/${id}/collections`, { params });
  }
}
