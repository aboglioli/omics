import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

import { ConfigService } from './config';
import { IUser } from '../models';

export interface IRegisterCommand {
  username: string;
  email: string;
  password: string;
}

export interface IRegisterResponse {
  id: string;
}

export interface ILoginCommand {
  username: string;
  password: string;
}

export interface ILoginResponse {
  auth_token: string;
}

export interface IUpdateCommand {
  name: string;
  lastname: string;
}

export interface IChangePasswordCommand {
  old_password: string;
  new_password: string;
}

@Injectable()
export class IdentityService {
  private baseUrl: string;

  constructor(private http: HttpClient, configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/users`;
  }

  public getById(id: string): Observable<IUser> {
    return this.http.get<IUser>(`${this.baseUrl}/${id}`);
  }

  public register(cmd: IRegisterCommand): Observable<IRegisterResponse> {
    return this.http.post<IRegisterResponse>(this.baseUrl, cmd);
  }

  public login(cmd: ILoginCommand): Observable<ILoginResponse> {
    return this.http.post<ILoginResponse>(this.baseUrl, cmd);
  }

  public update(cmd: IUpdateCommand): Observable<any> {
    return this.http.put(this.baseUrl, cmd);
  }

  public delete(id: string): Observable<any> {
    return this.http.delete(`${this.baseUrl}/${id}`);
  }

  public changePassword(cmd: IChangePasswordCommand): Observable<any> {
    return this.http.put(this.baseUrl, cmd);
  }

  public recoverPassword(email: string): Observable<any> {
    return this.http.get(`${this.baseUrl}/${email}`);
  }
}
