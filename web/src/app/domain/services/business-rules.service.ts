import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config.service';
import { IBusinessRules, } from '../models';

@Injectable()
export class BusinessRulesService {
  private baseUrl: string;

  constructor(private http: HttpClient, configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/configuration`;
  }

  public get(): Observable<IBusinessRules> {
    return this.http.get<IBusinessRules>(`${this.baseUrl}`);
  }

  public save(cmd: IBusinessRules): Observable<any> {
    return this.http.put(`${this.baseUrl}`, cmd);
  }
}
