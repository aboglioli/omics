import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config';
import { ICollection } from '../models';
import { ITag } from '../models/publication';

export interface ISearchCommand {
  author_id?: string;
  category_id?: string;
  status?: string;
  text?: string;
}

export interface ISearchResponse {
  collections: ICollection[];
}

export interface ICreateCommand {
  name: string;
  synopsis: string;
  category_id: string;
  tags: ITag[];
  cover: string;
}

export interface ICreateResponse {
  id: string;
}

export interface IUpdateCommand {
  name: string;
  synopsis: string;
  category_id: string;
  tags: ITag[];
  cover: string;
}

@Injectable()
export class CollectionService {
  private baseUrl: string;

  constructor(private http: HttpClient, configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/collections`;
  }

  public getById(id: string): Observable<ICollection> {
    return this.http.get<ICollection>(`${this.baseUrl}/${id}`);
  }

  public search(cmd: ISearchCommand): Observable<ISearchResponse> {
    let params = new HttpParams();

    if (cmd.author_id) {
      params = params.append('author_id', cmd.author_id);
    }

    if (cmd.category_id) {
      params = params.append('category_id', cmd.category_id);
    }

    if (cmd.status) {
      params = params.append('status', cmd.status);
    }

    if (cmd.text) {
      params = params.append('text', cmd.text);
    }

    return this.http.get<ISearchResponse>(`${this.baseUrl}`, { params });
  }

  public create(cmd: ICreateCommand): Observable<ICreateResponse> {
    return this.http.post<ICreateResponse>(`${this.baseUrl}`, cmd);
  }

  public update(id: string, cmd: IUpdateCommand): Observable<any> {
    return this.http.put(`${this.baseUrl}/${id}`, cmd);
  }

  public delete(id: string): Observable<any> {
    return this.http.delete(`${this.baseUrl}/${id}`);
  }

  public addPublication(id: string, publicationId: string): Observable<any> {
    return this.http.post(`${this.baseUrl}/${id}/publication/${publicationId}`, {});
  }

  public removePublication(id: string, publicationId: string): Observable<any> {
    return this.http.delete(`${this.baseUrl}/${id}/publication/${publicationId}`, {});
  }
}
