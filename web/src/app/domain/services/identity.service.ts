import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config.service';
import { IUser } from '../models';

export interface ISearchCommand {
  role_id?: string;
}

export interface ISearchResponse {
  users: IUser[];
}

export interface IRegisterCommand {
  username: string;
  email: string;
  password: string;
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

export interface IUpdateCommand {
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

  public search(cmd: ISearchCommand, include: string = ''): Observable<ISearchResponse> {
    let params = new HttpParams();

    if (cmd.role_id) {
      params = params.append('role_id', cmd.role_id);
    }

    if (include) {
      params = params.append('include', include);
    }

    return this.http.get<ISearchResponse>(`${this.baseUrl}`, { params });
  }

  public register(cmd: IRegisterCommand): Observable<IRegisterResponse> {
    return this.http.post<IRegisterResponse>(`${this.configServ.baseUrl()}/register`, cmd);
  }

  public login(cmd: ILoginCommand): Observable<ILoginResponse> {
    return this.http.post<ILoginResponse>(`${this.configServ.baseUrl()}/login`, cmd);
  }

  public update(id: string, cmd: IUpdateCommand): Observable<any> {
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
}
