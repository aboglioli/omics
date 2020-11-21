import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config.service';
import {
  IPagination,
  IPermission,
  IRole,
  IUser,
} from '../models';

export interface IGetAllResponse {
  roles: IRole[];
}

export interface IGetPermissionsResponse {
  permissions: IPermission[];
}

export interface ICreateCommand {
  name: string;
  permissions: string[];
}

export interface ICreateResponse {
  id: string;
}

export interface IUpdateCommand {
  name: string;
  permissions: string[];
}

@Injectable()
export class RoleService {
  private baseUrl: string;

  constructor(private http: HttpClient, configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/roles`;
  }

  public getAll(): Observable<IGetAllResponse> {
    return this.http.get<IGetAllResponse>(`${this.baseUrl}`);
  }

  public getPermissions(): Observable<IGetPermissionsResponse> {
    return this.http.get<IGetPermissionsResponse>(`${this.baseUrl}/permissions`);
  }

  public getById(id: string): Observable<IRole> {
    return this.http.get<IRole>(`${this.baseUrl}/${id}`);
  }

  public getUsers(id: string, include: string = ''): Observable<IPagination<IUser>> {
    let params = new HttpParams();

    if (include) {
      params = params.append('include', include);
    }

    return this.http.get<IPagination<IUser>>(`${this.baseUrl}/${id}/users`, { params });
  }

  public create(cmd: ICreateCommand): Observable<ICreateResponse> {
    return this.http.post<ICreateResponse>(`${this.baseUrl}`, cmd);
  }

  public update(id: string, cmd: IUpdateCommand): Observable<any> {
    return this.http.put(`${this.baseUrl}/${id}`, cmd);
  }

  public delete(id: string): Observable<any> {
    return this.http.delete(`${this.baseUrl}/${id}`);
  }

  public makeDefault(id: string): Observable<any> {
    return this.http.put(`${this.baseUrl}/${id}/default`, {});
  }
}
