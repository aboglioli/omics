import { HttpClient, HttpParams, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config';

export interface IImageFile {
  url: string;
}

export interface IUploadResponse {
  files: IImageFile[];
}

@Injectable()
export class FileService {
  private baseUrl: string;

  constructor(private http: HttpClient, configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/upload`;
  }

  public upload(data: FormData): Observable<IUploadResponse> {
    return this.http.post<IUploadResponse>(`${this.baseUrl}`, data);
  }
}
