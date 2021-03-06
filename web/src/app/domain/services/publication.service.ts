import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config.service';
import {
  ICollection,
  IContract,
  IPage,
  IPagination,
  IPublication,
  IReaderPublicationInteraction,
  IReview,
  IStatistics,
} from '../models';

export interface IGetByIdResponse {
  publication: IPublication;
  reader?: IReaderPublicationInteraction;
}

export interface ISearchCommand {
  author_id?: string;
  category_id?: string;
  status?: string;
  tag?: string;
  name?: string;
  date_from?: string;
  date_to?: string;
  offset?: number;
  limit?: number;
  order_by?: string; // 'most_viewed' 'most_liked' 'newest' 'best_reviews'
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

export interface IRequestContractResponse {
  id: string;
}

export interface ICanRequestContractResponse {
  can_request: boolean;
}

export interface IGetStatisticsCommand {
  date_from: string;
  date_to: string;
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

  public search(cmd: ISearchCommand, include: string = ''): Observable<IPagination<IPublication>> {
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

    return this.http.get<IPagination<IPublication>>(`${this.baseUrl}`, { params });
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

  public getCollections(id: string, include: string = ''): Observable<IPagination<ICollection>> {
    let params = new HttpParams();

    if (include) {
      params = params.append('include', include);
    }

    return this.http.get<IPagination<ICollection>>(`${this.baseUrl}/${id}/collections`, { params });
  }

  public addToFavorites(id: string): Observable<any> {
    return this.http.post(`${this.baseUrl}/${id}/favorite`, {});
  }

  public removeFromFavorites(id: string): Observable<any> {
    return this.http.delete(`${this.baseUrl}/${id}/favorite`);
  }

  public getStatistics(id: string, cmd: IGetStatisticsCommand): Observable<IStatistics> {
    let params = new HttpParams();

    if (cmd.date_from) {
      params = params.append('date_from', cmd.date_from);
    }

    if (cmd.date_to) {
      params = params.append('date_to', cmd.date_to);
    }

    return this.http.get<IStatistics>(`${this.baseUrl}/${id}/statistics`, { params });
  }

  public getContract(id: string): Observable<IContract> {
    return this.http.get<IContract>(`${this.baseUrl}/${id}/contract`);
  }

  public generateSummaries(id: string): Observable<IContract> {
    return this.http.get<IContract>(`${this.baseUrl}/${id}/contract/summaries`);
  }

  public canRequestContract(id: string): Observable<ICanRequestContractResponse> {
    return this.http.get<ICanRequestContractResponse>(`${this.baseUrl}/${id}/contract/can-request`);
  }

  public requestContract(id: string): Observable<IRequestContractResponse> {
    return this.http.post<IRequestContractResponse>(`${this.baseUrl}/${id}/contract`, {});
  }
}
