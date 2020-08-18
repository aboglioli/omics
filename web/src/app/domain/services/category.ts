import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config';
import { ICategory } from '../models';

export interface IGetAllResponse {
  categories: ICategory[];
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
}
