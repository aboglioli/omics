import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigService } from './config.service';
import { IReport } from '../models';

export interface IGenerateCommand {
  date_from: string;
  date_to: string;
}

@Injectable()
export class ReportService {
  private baseUrl: string;

  constructor(private http: HttpClient, configServ: ConfigService) {
    this.baseUrl = `${configServ.baseUrl()}/reports`;
  }

  public generate(cmd: IGenerateCommand): Observable<IReport> {
    let params = new HttpParams();
    params = params.append('date_from', cmd.date_from);
    params = params.append('date_to', cmd.date_to);

    return this.http.get<IReport>(`${this.baseUrl}`, { params });
  }
}
