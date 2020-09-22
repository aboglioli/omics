import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config.service';
import { INotification } from '../models';

export interface IFilterCommand {
  read?: boolean;
}

export interface IGetAllResponse {
  notifications: INotification[];
}

@Injectable()
export class NotificationService {
  private baseUrl: string;

  constructor(private http: HttpClient, configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/notifications`;
  }

  public getAll(cmd: IFilterCommand): Observable<IGetAllResponse> {
    let params = new HttpParams();

    if (cmd.read) {
      params = params.append('read', cmd.read ? 'true' : 'false');
    }

    return this.http.get<IGetAllResponse>(`${this.baseUrl}`, { params });
  }

  public markAllAsRead(): Observable<any> {
    return this.http.post(`${this.baseUrl}/read`, {});
  }
}
