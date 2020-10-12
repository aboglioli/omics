import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config.service';
import { ICollection, IPublication } from '../models';

export interface ISearchCommand {
  author_id?: string;
  category_id?: string;
  publication_id?: string;
  tag?: string;
  name?: string;
  date_from?: string;
  date_to?: string;
  offset?: number;
  limit?: number;
  order_by?: string;
}

export interface ISearchResponse {
  collections: ICollection[];
}

export interface ICreateCommand {
  name: string;
  synopsis: string;
  category_id: string;
  tags: string[];
  cover: string;
}

export interface ICreateResponse {
  id: string;
}

export interface IUpdateCommandCollection {
  name: string;
  synopsis: string;
  category_id: string;
  tags: string[];
  cover: string;
}

export interface IGetPublicationsResponse {
  publications: IPublication[];
}

@Injectable()
export class CollectionService {
  private baseUrl: string;

  constructor(private http: HttpClient, configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/collections`;
  }

  public getById(id: string, include: string = ''): Observable<ICollection> {
    let params = new HttpParams();

    if (include) {
      params = params.append('include', include);
    }

    return this.http.get<ICollection>(`${this.baseUrl}/${id}`, { params });
  }

  public getPublications(id: string, include: string = ''): Observable<IGetPublicationsResponse> {
    let params = new HttpParams();

    if (include) {
      params = params.append('include', include);
    }
    return this.http.get<IGetPublicationsResponse>(`${this.baseUrl}/${id}/publications`, { params });
  }

  public search(cmd: ISearchCommand, include: string = ''): Observable<ISearchResponse> {
    let params = new HttpParams();

    if (cmd.author_id) {
      params = params.append('author_id', cmd.author_id);
    }

    if (cmd.category_id) {
      params = params.append('category_id', cmd.category_id);
    }

    if (cmd.publication_id) {
      params = params.append('publication_id', cmd.publication_id);
    }

    if (cmd.tag) {
      params = params.append('tag', cmd.tag);
    }

    if (cmd.name) {
      params = params.append('name', cmd.name);
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

    return this.http.get<ISearchResponse>(`${this.baseUrl}`, { params });
  }

  public create(cmd: ICreateCommand): Observable<ICreateResponse> {
    return this.http.post<ICreateResponse>(`${this.baseUrl}`, cmd);
  }

  public update(id: string, cmd: IUpdateCommandCollection): Observable<any> {
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

  public addToFavorites(id: string): Observable<any> {
    return this.http.post(`${this.baseUrl}/${id}/favorite`, {});
  }

  public removeFromFavorites(id: string): Observable<any> {
    return this.http.delete(`${this.baseUrl}/${id}/favorite`);
  }
}
