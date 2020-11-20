import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config.service';
import { IPagination, IUser } from '../models';

export interface ISearchCommand {
  role_id?: string;
  date_from?: string;
  date_to?: string;
  offset?: number;
  limit?: number;
  order_by?: string; // 'newest', 'oldest'
}

export interface ISearchResponse {
  users: IUser[];
}

export interface IRegisterCommand {
  username: string;
  email: string;
  password: string;
  birthdate: string; // RFC 3339
}

export interface IRegisterResponse {
  id: string;
  validation_code: string;
}

export interface ILoginCommand {
  username: string;
  password: string;
}

export interface ILoginResponse {
  user_id: string;
  auth_token: string;
}

export interface IUpdateCommandUser {
  name: string;
  lastname: string;
  birthdate?: string; // RFC 3339
  gender?: string; // male, female, other
  biography?: string;
  profile_image?: string;
}

export interface IChangePasswordCommand {
  old_password: string;
  new_password: string;
}

export interface IRecoverPasswordCommand {
  email: string;
}

export interface IChangePaymentEmailCommand {
  payment_email: string;
}

@Injectable()
export class IdentityService {
  private baseUrl: string;

  constructor(private http: HttpClient, private configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/users`;
  }

  public getById(id: string, include: string = ''): Observable<IUser> {
    let params = new HttpParams();

    if (include) {
      params = params.append('include', include);
    }

    return this.http.get<IUser>(`${this.baseUrl}/${id}`, { params });
  }

  public search(cmd: ISearchCommand, include: string = ''): Observable<IPagination<IUser>> {
    let params = new HttpParams();

    if (cmd.role_id) {
      params = params.append('role_id', cmd.role_id);
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

    return this.http.get<IPagination<IUser>>(`${this.baseUrl}`, { params });
  }

  public register(cmd: IRegisterCommand): Observable<IRegisterResponse> {
    return this.http.post<IRegisterResponse>(`${this.configServ.baseUrl()}/register`, cmd);
  }

  public login(cmd: ILoginCommand): Observable<ILoginResponse> {
    return this.http.post<ILoginResponse>(`${this.configServ.baseUrl()}/login`, cmd);
  }

  public update(id: string, cmd: IUpdateCommandUser): Observable<any> {
    return this.http.put(`${this.baseUrl}/${id}`, cmd);
  }

  public delete(id: string): Observable<any> {
    return this.http.delete(`${this.baseUrl}/${id}`);
  }

  public changePassword(id: string, cmd: IChangePasswordCommand): Observable<any> {
    return this.http.put(`${this.baseUrl}/${id}/password`, cmd);
  }

  public recoverPassword(cmd: IRecoverPasswordCommand): Observable<any> {
    return this.http.post(`${this.configServ.baseUrl()}/recover-password`, cmd);
  }

  public changePaymentEmail(id: string, cmd: IChangePaymentEmailCommand): Observable<any> {
    return this.http.put(`${this.baseUrl}/${id}/payment-email`, cmd);
  }
}
