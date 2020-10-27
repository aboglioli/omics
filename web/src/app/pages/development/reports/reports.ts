import { Component, OnInit } from '@angular/core';

import { ReportService } from '../../../domain/services/report.service';
import { IReport } from '../../../domain/models/repor';

@Component({
  selector: 'dev-reports',
  templateUrl: './reports.html',
  styleUrls: ['../general.scss'],
})
export class DevReportsComponent implements OnInit {

  constructor(
  ) { }

  ngOnInit(): void {
  }
}
