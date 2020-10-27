import { Component, OnInit } from '@angular/core';
import { faChartLine, faFileCsv, faFilePdf } from '@fortawesome/free-solid-svg-icons';
import { ChartOptions } from 'chart.js';
import { Label, ThemeService } from 'ng2-charts';
import pdfMake from 'pdfmake';

import { IReport } from '../../domain/models/report';
import { ReportService } from '../../domain/services/report.service';
import { FormBuilder, FormGroup } from '@angular/forms';
import { NgxSpinnerService } from 'ngx-spinner';
import { ChartDataClassPie, ChartDataClassBar } from '../../models/chart-data.model';

pdfMake.fonts = {
   Roboto: {
     normal: 'https://cdnjs.cloudflare.com/ajax/libs/pdfmake/0.1.66/fonts/Roboto/Roboto-Regular.ttf',
     bold: 'https://cdnjs.cloudflare.com/ajax/libs/pdfmake/0.1.66/fonts/Roboto/Roboto-Medium.ttf',
     italics: 'https://cdnjs.cloudflare.com/ajax/libs/pdfmake/0.1.66/fonts/Roboto/Roboto-Italic.ttf',
     bolditalics: 'https://cdnjs.cloudflare.com/ajax/libs/pdfmake/0.1.66/fonts/Roboto/Roboto-MediumItalic.ttf'
   },
};

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
  public chartBarUsersByAgeRange: ChartDataClassBar = new ChartDataClassBar();

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
        // console.log('TEST > ', this.report);
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


    // #region Chart usuarios por género
    this.chartPieUsersByGender.type = 'pie';
    this.chartPieUsersByGender.legend = true;

    auxliarLabelValue = this.transformDataToChartValue( this.report.users.by_gender);

    this.chartPieUsersByGender.labels = auxliarLabelValue.labels;
    this.chartPieUsersByGender.values = auxliarLabelValue.values;
    this.chartPieUsersByGender.plugins = [ auxliarLabelValue.labels ];

    this.chartPieUsersByGender.options = this.defaultPlotOptions;

    //#endregion


    // #region Chart Usuarios por rango de edad
    this.chartBarUsersByAgeRange.type = 'bar';
    this.chartBarUsersByAgeRange.legend = true;

    auxliarLabelValue = this.transformDataToChartValue( this.report.users.by_age);

    this.chartBarUsersByAgeRange.labels = auxliarLabelValue.labels;
    this.chartBarUsersByAgeRange.data = [ {data: auxliarLabelValue.values, label: 'Cantidad' }];
    this.chartBarUsersByAgeRange.plugins = [ auxliarLabelValue.labels ];

    this.chartBarUsersByAgeRange.options = this.defaultPlotOptions;

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
    if (!this.report) {
      return;
    }

    const dateFrom = new Date(this.formReport.get('dateFrom').value);
    const dateTo = new Date( this.formReport.get('dateTo').value );

    const capitalize = (str) => str.charAt(0).toUpperCase() + str.slice(1);

    const mapItem = (item) => Object.keys(item)
      .map((key) => ({
        text: [
          { text: capitalize(key), bold: true },
          ': ',
          item[key],
        ],
      }));

    const docDefinition = {
      content: [
        { text: 'Reporte de Omics', fontSize: 24 },
        `Desde: ${dateFrom.toISOString()} - Hasta: ${dateTo.toISOString()}`,

        { text: 'Usuarios', bold: true, margin: [0, 5, 0, 0], fontSize: 18 },
        { text: `Total: ${this.report.users.total}` },
        { text: 'Por Estado', fontSize: 13, margin: [0, 10, 0, 0] },
        {
          ul: mapItem(this.report.users.by_status),
          margin: [0, 0, 0, 5],
        },
        { text: 'Por Género', fontSize: 13, margin: [0, 3, 0, 0] },
        {
          ul: mapItem(this.report.users.by_gender),
          margin: [0, 0, 0, 5],
        },
        { text: 'Por Edad', fontSize: 13, margin: [0, 3, 0, 0] },
        {
          ul: mapItem(this.report.users.by_age),
          margin: [0, 0, 0, 5],
        },

        { text: 'Publicaciones', bold: true, margin: [0, 5, 0, 0], fontSize: 18 },
        { text: `Total: ${this.report.publications.total}` },
        { text: 'Por Categoría', fontSize: 13, margin: [0, 10, 0, 0] },
        {
          ul: mapItem(this.report.publications.by_category),
          margin: [0, 0, 0, 5],
        },
        { text: 'Por Contrato', fontSize: 13, margin: [0, 3, 0, 0] },
        {
          ul: mapItem(this.report.publications.by_contract),
          margin: [0, 0, 0, 5],
        },
        { text: 'Por Estado', fontSize: 13, margin: [0, 3, 0, 0] },
        {
          ul: mapItem(this.report.publications.by_status),
          margin: [0, 0, 0, 5],
        },
        { text: 'Por Cantidad de Páginas', fontSize: 13, margin: [0, 3, 0, 0] },
        {
          ul: mapItem(this.report.publications.by_pages),
          margin: [0, 0, 0, 5],
        },

        { text: 'Suscripciones', bold: true, margin: [0, 5, 0, 0], fontSize: 18 },
        { text: `Total: ${this.report.subscriptions.total}` },
        { text: 'Por Estado', fontSize: 13, margin: [0, 10, 0, 0] },
        {
          ul: mapItem(this.report.subscriptions.by_status),
          margin: [0, 0, 0, 5],
        },

        { text: 'Contratos', bold: true, margin: [0, 5, 0, 0], fontSize: 18 },
        { text: `Total: ${this.report.contracts.total}` },
        { text: 'Por Estado', fontSize: 13, margin: [0, 10, 0, 0] },
        {
          ul: mapItem(this.report.contracts.by_status),
          margin: [0, 0, 0, 5],
        },
        { text: 'Por Monto cobrado', fontSize: 13, margin: [0, 3, 0, 0] },
        {
          ul: mapItem(this.report.contracts.by_amount),
          margin: [0, 0, 0, 5],
        },
        { text: 'Por Cantidad de Pagos', fontSize: 13, margin: [0, 3, 0, 0] },
        {
          ul: mapItem(this.report.contracts.by_payment),
          margin: [0, 0, 0, 5],
        },

        { text: 'Donaciones', bold: true, margin: [0, 5, 0, 0], fontSize: 18 },
        { text: `Total: ${this.report.donations.total}` },
        { text: 'Por Estado', fontSize: 13, margin: [0, 10, 0, 0] },
        {
          ul: mapItem(this.report.donations.by_status),
          margin: [0, 0, 0, 5],
        },
        { text: 'Por Monto', fontSize: 13, margin: [0, 3, 0, 0] },
        {
          ul: mapItem(this.report.donations.by_amount),
          margin: [0, 0, 0, 5],
        },

        { text: 'Pagos', bold: true, margin: [0, 5, 0, 0], fontSize: 18 },
        {
          ul: [
            {
              text: [
                { text: 'Ingresos', bold: true },
                ': ',
                `\$${this.report.payments.income.toFixed(2)}`,
              ],
            },
            {
              text: [
                { text: 'Egresos', bold: true },
                ': ',
                `\$${this.report.payments.outcome.toFixed(2)}`,
              ],
            }
          ],
          margin: [0, 0, 0, 5],
        },
      ]
    };

    pdfMake.createPdf(docDefinition).open();
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
