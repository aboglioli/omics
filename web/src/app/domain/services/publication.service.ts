import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config.service';
import { IPublication, IReview, IPage, IReaderPublicationInteraction, ICollection } from '../models';

export interface IGetByIdResponse {
  publication: IPublication;
  reader?: IReaderPublicationInteraction;
}

export interface ISearchCommand {
  author_id?: string;
  category_id?: string;
  status?: string;
  name?: string;
}

export interface ISearchResponse {
  publications: IPublication[];
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

export interface IUpdateCommand {
  name: string;
  synopsis: string;
  category_id: string;
  tags: string[];
  cover: string;
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

export interface IReadResponse {
  id: string;
  pages: IPage[];
}

export interface IApproveCommand {
  comment: string;
}

export interface IRejectCommand {
  comment: string;
}

export interface IGetCollectionsResponse {
  collections: ICollection[];
}

@Injectable()
export class PublicationService {
  private baseUrl: string;

  constructor(private http: HttpClient, configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/publications`;
  }

  public getById(id: string, include: string = ''): Observable<IGetByIdResponse> {
    let params = new HttpParams();

    if (include) {
      params = params.append('include', include);
    }

    return this.http.get<IGetByIdResponse>(`${this.baseUrl}/${id}`, { params });
  }

  public search(cmd: ISearchCommand, include: string = ''): Observable<ISearchResponse> {
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

    if (cmd.name) {
      params = params.append('name', cmd.name);
    }

    if (include) {
      params = params.append('include', include);
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

  public approve(id: string, cmd: IApproveCommand): Observable<any> {
    return this.http.post(`${this.baseUrl}/${id}/approve`, cmd);
  }

  public reject(id: string, cmd: IRejectCommand): Observable<any> {
    return this.http.post(`${this.baseUrl}/${id}/reject`, cmd);
  }

  public read(id: string): Observable<IReadResponse> {
    return this.http.get<IReadResponse>(`${this.baseUrl}/${id}/read`);
  }

  public like(id: string): Observable<any> {
    return this.http.post(`${this.baseUrl}/${id}/like`, {});
  }

  public unlike(id: string): Observable<any> {
    return this.http.post(`${this.baseUrl}/${id}/unlike`, {});
  }

  public addReview(id: string, cmd: IAddReviewCommand): Observable<any> {
    return this.http.post(`${this.baseUrl}/${id}/review`, cmd);
  }

  public deleteReview(id: string): Observable<any> {
    return this.http.delete(`${this.baseUrl}/${id}/review`);
  }

  public getReviews(id: string): Observable<IGetReviewsResponse> {
    return this.http.get<IGetReviewsResponse>(`${this.baseUrl}/${id}/reviews`);
  }

  public getCollections(id: string, include: string = ''): Observable<IGetCollectionsResponse> {
    let params = new HttpParams();

    if (include) {
      params = params.append('include', include);
    }

    return this.http.get<IGetCollectionsResponse>(`${this.baseUrl}/${id}/collections`, { params });
  }

  public addToFavorites(id: string): Observable<any> {
    return this.http.post(`${this.baseUrl}/${id}/favorite`, {});
  }

  public removeFromFavorites(id: string): Observable<any> {
    return this.http.delete(`${this.baseUrl}/${id}/favorite`);
  }
}
