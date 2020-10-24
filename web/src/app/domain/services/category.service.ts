import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config.service';
import {
  ICategory,
  ICollection,
  IPagination,
  IPublication,
} from '../models';

export interface IGetAllResponse {
  categories: ICategory[];
}

export interface ICreateCommand {
  id: string;
  name: string;
}

export interface ICreateResponse {
  id: string;
}

export interface IUpdateCommand {
  name: string;
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

  public getPublications(id: string, include: string = ''): Observable<IPagination<IPublication>> {
    let params = new HttpParams();

    if (include) {
      params = params.append('include', include);
    }

    return this.http.get<IPagination<IPublication>>(`${this.baseUrl}/${id}/publications`, { params });
  }

  public getCollections(id: string, include: string = ''): Observable<IPagination<ICollection>> {
    let params = new HttpParams();

    if (include) {
      params = params.append('include', include);
    }

    return this.http.get<IPagination<ICollection>>(`${this.baseUrl}/${id}/collections`, { params });
  }

  public create(cmd: ICreateCommand): Observable<any> {
    return this.http.post<ICreateResponse>(`${this.baseUrl}`, cmd);
  }

  public update(id: string, cmd: IUpdateCommand): Observable<any> {
    return this.http.put(`${this.baseUrl}/${id}`, cmd);
  }

  public delete(id: string): Observable<any> {
    return this.http.delete(`${this.baseUrl}/${id}`);
  }
}
