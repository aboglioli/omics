import { Component, OnInit } from '@angular/core';
import { faChartLine, faFileCsv, faFilePdf } from '@fortawesome/free-solid-svg-icons';
import { ChartOptions, ChartType } from 'chart.js';
import { Label, ThemeService } from 'ng2-charts';

import { IReport } from '../../domain/models/report';
import { ReportService } from '../../domain/services/report.service';
import { FormBuilder, FormGroup } from '@angular/forms';
import { NgxSpinnerService } from 'ngx-spinner';
import { ChartDataClassPie, ChartDataClassBar } from '../../models/chart-data.model';


@Component({
  selector: 'app-dashboard-reportes',
  templateUrl: './dashboard-reportes.component.html',
  styleUrls: ['./dashboard-reportes.component.scss']
})
export class DashboardReportesComponent implements OnInit {


  // Font Awseome icons
  public faChart = faChartLine;
  public faPDF = faFilePdf;
  public faCSV = faFileCsv;

  // Generales
  public report: IReport;

  // De Formulario
  public formReport: FormGroup;
  public maxDateToSearch: Date = new Date();

  // De Charts
  public defaultPlotOptions: ChartOptions = {

    responsive: true,
    legend: {
      position: 'top'
    },
    plugins: {
      datalabels: {
        formatter: (value, ctx) => {
          const label = ctx.chart.data.labels[ctx.dataIndex];
          return label;
        },
      },
    }

  };

  public chartPiePublicationByCategory: ChartDataClassPie = new ChartDataClassPie();
  public chartPiePublicationByStatus: ChartDataClassPie = new ChartDataClassPie();
  public chartPiePublicationByContract: ChartDataClassPie = new ChartDataClassPie();

  public chartBarContractByAmount: ChartDataClassBar = new ChartDataClassBar();

  public chartPieUsersByGender: ChartDataClassPie = new ChartDataClassPie();

  constructor(
    private reportService: ReportService,
    private fb: FormBuilder,
    private spinnerService: NgxSpinnerService,
    private themeService: ThemeService
  ) { }

  ngOnInit(): void {

    this.setDarkThemeCharts();

    this.createFormDateRange();
    this.generateChartsData();

  }

  private createFormDateRange(): void {


    const dateTo = new Date();
    const dateFrom = new Date( dateTo );
    dateFrom.setHours(dateTo.getHours() - 24 * 60);

    this.formReport = this.fb.group({
      dateFrom:      [ dateFrom ],
      dateTo:        [ dateTo ],
    });

  }

  public convertDateToRFC3339(changeDate: Date, controlName: string): void {

    if ( controlName === 'dateTo'){

      changeDate = new Date(changeDate);
      changeDate.setHours( 23 );

    }
    this.formReport.get(controlName).setValue( changeDate.toISOString() );

  }

  public generateChartsData(): void {

    const dateFrom = new Date(this.formReport.get('dateFrom').value);
    const dateTo = new Date( this.formReport.get('dateTo').value );

    this.spinnerService.show();

    this.reportService.generate({
      date_from: dateFrom.toISOString(),
      date_to: dateTo.toISOString(),
    }).subscribe(
      (res) => {

        this.spinnerService.hide();

        this.report = res;
        console.log('TEST > ', this.report);
        this.setCharts( );

      },
      (err: Error ) => {
        this.spinnerService.hide();
        console.error('ERROR: ', err);
      }
    );

  }

  private setCharts(): void {

    let auxliarLabelValue;

    // #region Chart Publicaciones por Categoría
    this.chartPiePublicationByCategory.type = 'pie';
    this.chartPiePublicationByCategory.legend = true;

    auxliarLabelValue = this.transformDataToChartValue( this.report.publications.by_category);

    this.chartPiePublicationByCategory.labels = auxliarLabelValue.labels;
    this.chartPiePublicationByCategory.values = auxliarLabelValue.values;
    this.chartPiePublicationByCategory.plugins = [ auxliarLabelValue.labels ];

    this.chartPiePublicationByCategory.options = this.defaultPlotOptions;

    //#endregion

    // #region Chart Publicaciones estado
    this.chartPiePublicationByStatus.type = 'pie';
    this.chartPiePublicationByStatus.legend = true;

    auxliarLabelValue = this.transformDataToChartValue( this.report.publications.by_status);

    this.chartPiePublicationByStatus.labels = auxliarLabelValue.labels;
    this.chartPiePublicationByStatus.values = auxliarLabelValue.values;
    this.chartPiePublicationByStatus.plugins = [ auxliarLabelValue.labels ];

    this.chartPiePublicationByStatus.options = this.defaultPlotOptions;

    //#endregion

    // #region Chart Publicaciones por Contrato o no contrato
    this.chartPiePublicationByContract.type = 'pie';
    this.chartPiePublicationByContract.legend = true;

    auxliarLabelValue = this.transformDataToChartValue( this.report.publications.by_contract);

    this.chartPiePublicationByContract.labels = auxliarLabelValue.labels;
    this.chartPiePublicationByContract.values = auxliarLabelValue.values;
    this.chartPiePublicationByContract.plugins = [ auxliarLabelValue.labels ];

    this.chartPiePublicationByContract.options = this.defaultPlotOptions;

    //#endregion


    // #region Chart Contratos por rango de cantidad recibido
    this.chartBarContractByAmount.type = 'bar';
    this.chartBarContractByAmount.legend = true;

    auxliarLabelValue = this.transformDataToChartValue( this.report.contracts.by_amount);

    this.chartBarContractByAmount.labels = auxliarLabelValue.labels;
    this.chartBarContractByAmount.data = [ {data: auxliarLabelValue.values, label: 'Cantidad' }];
    this.chartBarContractByAmount.plugins = [ auxliarLabelValue.labels ];

    this.chartBarContractByAmount.options = this.defaultPlotOptions;

    //#endregion


    // #region Chart Publicaciones por Contrato o no contrato
    this.chartPieUsersByGender.type = 'pie';
    this.chartPieUsersByGender.legend = true;

    auxliarLabelValue = this.transformDataToChartValue( this.report.users.by_gender);

    this.chartPieUsersByGender.labels = auxliarLabelValue.labels;
    this.chartPieUsersByGender.values = auxliarLabelValue.values;
    this.chartPieUsersByGender.plugins = [ auxliarLabelValue.labels ];

    this.chartPieUsersByGender.options = this.defaultPlotOptions;

    //#endregion


  }


  private transformDataToChartValue( objectToMap: object ): { labels: Label[], values: number[] } {

    // console.log('TEST > ', objectToMap);

    const labels: Label[] = Object.keys( objectToMap );
    const values: number[] = Object.values( objectToMap );

    return {
      labels,
      values
    };

  }



  public onExportar( type: string ): void {

    switch (type) {

      case 'pdf': {
        this.exportarPDF();
        break;
      }

      case 'cvs': {
        this.exportarCVS();
        break;
      }

    }


  }

  private exportarPDF(): void {

  }

  private exportarCVS(): void {

  }

  private setDarkThemeCharts(): void {

    let overrides: ChartOptions;
    overrides = {
      legend: {
        labels: { fontColor: 'white' }
      },
      scales: {
        xAxes: [{
          ticks: { fontColor: 'white' },
          gridLines: { color: 'rgba(255,255,255,0.1)' }
        }],
        yAxes: [{
          ticks: { fontColor: 'white' },
          gridLines: { color: 'rgba(255,255,255,0.1)' }
        }]
      }
    };

    this.themeService.setColorschemesOptions(  overrides );

  }


}
