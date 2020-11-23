import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config.service';
import { IBackupFile } from '../models';

@Injectable()
export class BackupService {
  private baseUrl: string;

  constructor(private http: HttpClient, configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/backup`;
  }

  public list(): Observable<IBackupFile[]> {
    return this.http.get<IBackupFile[]>(`${this.baseUrl}`);
  }

  public generate(): Observable<any> {
    return this.http.post(`${this.baseUrl}`, {});
  }
}
