import { Component, OnInit } from '@angular/core';
import { ChartDataSets, ChartOptions, ChartType, ChartColor } from 'chart.js';
import { Label } from 'ng2-charts';

import { IReport } from '../../domain/models/report';
import { ReportService } from '../../domain/services/report.service';

@Component({
  selector: 'app-dashboard-reportes',
  templateUrl: './dashboard-reportes.component.html',
  styleUrls: ['./dashboard-reportes.component.scss']
})
export class DashboardReportesComponent implements OnInit {
  public report: IReport;

  public chartOptions: ChartOptions = {
    responsive: true,
    // scales: { xAxes: [{}], yAxes: [{}] },
  };

  public chartLabels: Label[] = [];
  public chartData: number[] = [];

  constructor(
    private reportService: ReportService,
  ) { }

  ngOnInit(): void {
    const dateFrom = new Date(Date.parse('2020-01-01T00:00:00Z'));
    const dateTo = new Date();

    this.reportService.generate({
      date_from: dateFrom.toISOString(),
      date_to: dateTo.toISOString(),
    }).subscribe(
      (res) => {
        const report = res;
        this.report = report;

        this.chartLabels = Object.keys(report.publications.by_category);
        this.chartData = Object.values(report.publications.by_category);
        console.log(this.chartLabels);
        console.log(this.chartData);

        // this.chartData = Object.keys(report.publications.by_category)
        //   .map((key) => {
        //     console.log(key, report.publications.by_category[key]);
        //     return {
        //       data: [report.publications.by_category[key]],
        //       label: key,
        //     }
        //   });
      },
    );
  }
}
