import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config';
import { IPublication, IReview, ITag } from '../models';

export interface ISearchCommand {
  author_id?: string;
  category_id?: string;
  status?: string;
  text?: string;
}

export interface ISearchResponse {
  publications: IPublication[];
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

export interface IPage {
  images: string[];
}

export interface IUpdatePagesCommand {
  pages: {
    images: string[];
  }[];
}

export interface IAddReviewCommand {
  stars: number;
  comment: string;
}

export interface IGetReviewsResponse {
  reviews: IReview[];
}

@Injectable()
export class PublicationService {
  private baseUrl: string;

  constructor(private http: HttpClient, configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/publications`;
  }

  public getById(id: string): Observable<IPublication> {
    return this.http.get<IPublication>(`${this.baseUrl}/${id}`);
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

  public updatePages(id: string, cmd: IUpdatePagesCommand): Observable<any> {
    return this.http.put(`${this.baseUrl}/${id}/pages`, cmd);
  }

  public delete(id: string): Observable<any> {
    return this.http.delete(`${this.baseUrl}/${id}`);
  }

  public publish(id: string): Observable<any> {
    return this.http.post(`${this.baseUrl}/${id}/publish`, {});
  }

  public approve(id: string): Observable<any> {
    return this.http.post(`${this.baseUrl}/${id}/approve`, {});
  }

  public reject(id: string): Observable<any> {
    return this.http.post(`${this.baseUrl}/${id}/reject`, {});
  }

  public read(id: string): Observable<any> {
    return this.http.post(`${this.baseUrl}/${id}/read`, {});
  }

  public like(id: string): Observable<any> {
    return this.http.post(`${this.baseUrl}/${id}/like`, {});
  }

  public unlike(id: string): Observable<any> {
    return this.http.post(`${this.baseUrl}/${id}/unlike`, {});
  }

  public add_review(id: string, cmd: IAddReviewCommand): Observable<any> {
    return this.http.post(`${this.baseUrl}/${id}/review`, cmd);
  }

  public delete_review(id: string): Observable<any> {
    return this.http.delete(`${this.baseUrl}/${id}/review`);
  }

  public getReviews(id: string): Observable<IGetReviewsResponse> {
    return this.http.get<IGetReviewsResponse>(`${this.baseUrl}/${id}/reviews`);
  }
}
