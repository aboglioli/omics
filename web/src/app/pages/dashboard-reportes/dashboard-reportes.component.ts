import { Component, OnInit } from '@angular/core';
import { faAlignCenter, faChartLine, faFileCsv, faFilePdf } from '@fortawesome/free-solid-svg-icons';
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

      case 'csv': {
        this.exportarCSV();
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

    // const mapItem = (item) => Object.keys(item)
    //   .map((key) => ({
    //     text: [
    //       { text: capitalize(key), bold: true },
    //       ': ',
    //       item[key],
    //     ],
    //   }));

    const mapItem = (arr) => arr
      .map(({ key, value }) => ({
          text: [
            { text: capitalize(key), bold: true },
            ': ',
            value,
          ],
      }));

    const toArr = (obj) => Object.keys(obj)
      .map((key) => ({
        key,
        value: obj[key],
      }));

    const sort = (arr) => arr
      .map((item, index, arr) => {
        let order = index;

        if (typeof item.key === 'number') {
          order = item.key;
        } else if (item.key.startsWith('+')) {
          order = +item.key.slice(1);
        } else if (item.key.includes('-')) {
          order = +item.key.split('-')[0];
        }

        return {
          ...item,
          order,
        }
      })
      .sort((a, b) => a.order - b.order);

    const docDefinition = {

      content: [

        // Imagen OMICS
        {image: 'data:image/jpeg;base64,/9j/4AAQSkZJRgABAQIAOwA7AAD/2wBDAAQDAwQDAwQEAwQFBAQFBgoHBgYGBg0JCggKDw0QEA8NDw4RExgUERIXEg4PFRwVFxkZGxsbEBQdHx0aHxgaGxr/2wBDAQQFBQYFBgwHBwwaEQ8RGhoaGhoaGhoaGhoaGhoaGhoaGhoaGhoaGhoaGhoaGhoaGhoaGhoaGhoaGhoaGhoaGhr/wAARCAEdAyQDASIAAhEBAxEB/8QAHQABAAMBAQEBAQEAAAAAAAAAAAcICQYFBAMCAf/EAGAQAAEDAwIDBAUGBwgLDgUFAAEAAgMEBREGBwgSIRMxQVEUImFxgQkVIzKRoUJScoKSsbIWGDM0YqLCwyQ3Q3R1g5OVo7PRFzVHVFVWY3OFlLTBxNIZNlPT5Gd2ptTw/8QAGwEBAAIDAQEAAAAAAAAAAAAAAAMEAgUGBwH/xAA3EQEAAgECAggEBQMEAwEAAAAAAQIDBBEFEhMhMUFRYZGxMnGh0QZCgcHhIlLwFBUjQxZi8VP/2gAMAwEAAhEDEQA/AIv3L3Nvm5+o6q6XyrlNMZXGjo+c9lTR/gta3uzjGT3k9Sv801u3rnSHINPapudLEz6sDpzLEP8AFvy37l4mrLb8y6pvltDeUUVwnp8Yxjkkc3HcPLyC8dek1xYpxxWKxy+Gz2mmDBbDWkUjl26o2jZZTTXGjrO2cjNR2u2X2IfWe1rqaV35zct/mKX9OcZ2h7mGM1BQ3SxTH6zjEKiIfnMPMf0FQxFSy8L0mT8u3y6mrzcC0Gb8nLPlO33hqlprdbROr+Qad1PbKyV/1YPSBHKf8W7DvuXYLH1dhprdbW2kOQad1Pc6OJn1YPSDJEP8W7LfuWry8D//ADv6x9mjz/hfvw5PWP3j7NUkVDtNcZ+trZyM1Fb7Zfoh9Z/IaaV35zfVH6CmDTXGfom58jNRW+52GU/WfyCpib+c31v5i1eXherxfl3+XW0efgevw/k5o8p3+0rIouM01u3obV/INP6ptlVK/wCrA6cRSn/Fvw77l2a11qWpO1o2aa+O+KeW9ZifONhERYIxERAREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREGYW/Vu+a949ZwYA57k+fp/0uJP6ajpTnxcW70Leq5T4x6dR0s/vxGI/6v2KDF6Npbc+npbyj2eyaC/SaTFbxrHt/AiIrS8IiICIiAut01ujrPSHINOamudDEz6sDahzov8AJuy37lySLG1K3ja0bsL46ZI5bxEx5xusdprjN1za+RmoKK2X+IfWe6I08p/OZ6o/QUwaa40dGXPkZqO13OxSn6z2tbUxN/Obh38xURRa3LwvSZfy7fLqabPwPQZvycs+XV/H0amab3g0Jq0tbYNVWuplf9WF84hlPujk5Xfcu2WPq6rTW5msNH8g01qW52+JndBHUuMXxjOWn4hazLwOP+u/rH2aPP8AheO3Dk9Y/ePs1YRUI01xl67tXIy/0ts1BEPrPkh9HlPudHho/QUwaa41NH3LkZqW0XOxyu73x8tVE33uHK77GLV5eF6vH+Xf5dbRZ+Ba/D+Tmjynf6dUrLouH05vHoLVhY2xartc8r/qwyzdhKfdHJyu+5dwCCMjqFrb0vjna0bfNpsmLJinlvWYnzjYREWCMREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREBERAREQEREFHuN23dlrjTdxxj0i1GDPn2crnf1qq8rn8clu7Sz6NuIB+gqamAnH47WOH+rP3qmC73hlubR08t4+r1fgd+fh2Py3j0mRERbNuxERAREQEREBERAREQEREBdRpvcfV2kC0aa1Hc7bG3uhiqXdl8YyeU/ELl0WNq1vG1o3YXpTJHLeImPPrWI01xka+tPIy+wWzUEQ+s6WDsJT7nR4aP0Spg01xq6SuHIzU1mudkld3viLaqJvvI5XfY0qi6LXZeGaTL+Xb5dTT5+CaDP+TafLq/j6NSNN7zaB1aGix6rtksrvqwzTdhKf8AFycrvuXcghwBaQQRkEeKx+XS6c3C1XpEt/c1qK52xjTnsoapwjPvZnlPxC1eTgcf9d/WPs0eb8Lx24cnrH7x9mrqKgWmuMbcCz8jL2y26giH1nT0/Yykex0eG/a0qYNNca+la/kZqex3KzSHvfA5tVEPaT6rvsaVq8vCtXj/AC7/ACaLPwHX4eynNHlO/wBO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f1dxwei5xau630Fp/cSzPtWrLfHW0xyY3npJC78dj+9p93f3HI6Kkm6vCjqfRT5q/STZdT2QEnELP7Khb/KjH1sebc+ZAXTaPiuLPEVyf02+ku24dx3Dqoimb+m/0n5T3fKfV6/BlroWbWdw0tWS8tNe4e0pw49BURAnA8uZhd7+RoV6VkZabpW6cvNHcrdI6luFvqGzQuxgskY7IyPeO5albda4odxdHWvUVrIDKuIdtEDkwzDo+M+52feMHxWr4zp+TJGaOye35/zDR/iTRzjzRqax1W6p+cfePZ1KIi55x4iIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgjncDY3RG5PPNqC0thuLh/vhRnsaj3lwGH/nhy5fabZe+7NX+rhsd+ivOkbieaopKqMxT08gGGyMIy156BrvqZGD3tAM3IrManNGOcXNvWe6ev8A+LtddqK4ZwTbek909cfp4foIiKspCIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiAiIgIiICIiD//2Q=='
        , fit: [150, 150]
        , alignment: 'center'
        },
        // Cabecera
        { text: `Usuario: admin-1`, alignment: 'right' },
        { text: `Rol: Admin`, alignment: 'right' },
        { text: `Fecha: ${new Date().toLocaleString()}`, alignment: 'right' },

        { text: '\nReporte General\n', style: 'header', alignment: 'center'},
        { text: `Desde: ${dateFrom.toLocaleString().split(' ')[0]} - Hasta: ${dateTo.toLocaleString().split(' ')[0]}`, alignment: 'center'},

        // Reporte de Publicaciones
        { text: 'Reporte de Publicaciones', style: 'subheader' },
        {
          style: 'tableExample',
          table: {
            headerRows: 1,
            widths: ['*', '*', '*', '*'],
            body: [
              [{text: 'Por Categoría', style: 'tableHeader'}, {text: 'Por Contrato', style: 'tableHeader'}, {text: 'Por Estado', style: 'tableHeader'}, {text: 'Por Cantidad de Páginas', style: 'tableHeader'}],
              [
                {style: 'tablebody', ul: mapItem(toArr(this.report.publications.by_category)) },
                {style: 'tablebody', ul: mapItem(toArr(this.report.publications.by_contract)) },
                {style: 'tablebody', ul: mapItem(toArr(this.report.publications.by_status)) },
                {style: 'tablebody', ul: mapItem(sort(toArr(this.report.publications.by_pages))) }
              ],
            ]
          }
        },

        {
          style: 'tableExample',
          table: {
            body: [
              [{text: 'Total de Publicaciones: ', style: 'tableResult'}, {text: `${this.report.publications.total}`}]
            ]
          },
          layout: 'dash'
        },
        // Reporte por Contratos
        { text: 'Reporte de Contratos', style: 'subheader'},
        {
          style: 'tableExample',
          table: {
            headerRows: 1,
            widths: ['*', '*', '*'],
            body: [
              // tslint:disable-next-line: max-line-length
              [{text: 'Por Estado', style: 'tableHeader'}, {text: 'Por Monto Cobrado', style: 'tableHeader'}, {text: 'Por Cantidad de Pagos', style: 'tableHeader'}],
              [
                {style: 'tablebody', ul: mapItem(toArr(this.report.contracts.by_status)) },
                {style: 'tablebody', ul: mapItem(sort(toArr(this.report.contracts.by_amount))) },
                {style: 'tablebody', ul: mapItem(sort(toArr(this.report.contracts.by_payment))) }
              ],
            ]
          }
        },

        {
          style: 'tableExample',
          table: {
            body: [
              [{text: 'Total de Contratos: ', style: 'tableResult'}, {text: `${this.report.contracts.total}`}]
            ]
          },
          layout: 'dash'
        },

        // Reporte de Usuarios
        { text: 'Reporte de Usuarios', style: 'subheader', pageBreak: 'before'},
        {
          style: 'tableExample',
          table: {
            headerRows: 1,
            widths: [200, 200, '*'],
            body: [
              [{text: 'Por Estado', style: 'tableHeader'}, {text: 'Por Género', style: 'tableHeader'}, {text: 'Por Edad', style: 'tableHeader'}],
              [
                {style: 'tablebody', ul: mapItem(toArr(this.report.users.by_status)) },
                {style: 'tablebody', ul: mapItem(toArr(this.report.users.by_gender)) },
                {style: 'tablebody', ul: mapItem(sort(toArr(this.report.users.by_age))) }

              ],
            ]
          }
        },

        {
          style: 'tableExample',
          table: {
            body: [
              [{text: 'Total de Usuarios: ', style: 'tableResult'}, {text: `${this.report.users.total}`}]
            ]
          },
          layout: 'dash'
        },

        // Reporte de Suscripciones y Donaciones
        { text: 'Reporte de Suscripciones y Donaciones', style: 'subheader'},
        {
          style: 'tableExample',
          table: {
            widths: ['*', '*', '*'],
            headerRows: 2,
            body: [
              [{text: 'Suscripciones', style: 'tableHeader'}, {text: 'Donaciones', style: 'tableHeader', colSpan: 2}, {}],
              [{text: 'Por Estado', style: 'tableRow'}, {text: 'Por Monto', style: 'tableRow'}, {text: 'Por Estado', style: 'tableRow'}],
              [

                {style: 'tablebody', ul: mapItem(toArr(this.report.subscriptions.by_status)) },
                {style: 'tablebody', ul: mapItem(sort(toArr(this.report.donations.by_amount))) },
                {style: 'tablebody', ul: mapItem(toArr(this.report.donations.by_status)) }

              ],
            ]
          }
        },

        {
          style: 'tableExample',
          table: {
            body: [
              [{text: 'Total de Suscripciones: ', style: 'tableResult'}, {text: `${this.report.subscriptions.total}`},
               {text: ' ', border: [false, false, false, false] },
               {text: 'Total de Donaciones: ', style: 'tableResult'}, {text: `${this.report.donations.total}`}
            ]
            ]
          },
          layout: 'dash'
               },

        // Reporte de Pagos
        { text: 'Reporte de Pagos', style: 'subheader', pageBreak: 'before'},
        {
          style: 'tableExample',
          table: {
            widths: ['*', '*', '*'],
            body: [
              [
                '',
                {text: 'Ingresos', style: 'tableHeader'},
                {text: 'Egresos', style: 'tableHeader'}
              ],
              [
                {text: 'Contratos', style: 'tableRow'},
                '$0',
                `\$${this.report.payments.contract_outcome.toFixed(2)}`,
              ],
              [
                {text: 'Suscripciones', style: 'tableRow'},
                `\$${this.report.payments.subscription_income.toFixed(2)}`,
                '$0'
              ],
              [
                {text: 'Donaciones', style: 'tableRow'},
                `\$${this.report.payments.donation_income.toFixed(2)}`,
                `\$${this.report.payments.donation_outcome.toFixed(2)}`,
              ],
              [
                {text: 'Total', style: 'tableRow'},
                `\$${this.report.payments.total_income.toFixed(2)}`,
                `\$${this.report.payments.total_outcome.toFixed(2)}`
              ]
            ]
          },
        },
      ],

      layout: {
        dash: {
          hLineWidth: function (i, node) {
            return (i === 0 || i === node.table.body.length) ? 2 : 1;
          },
          vLineWidth: function (i, node) {
            return (i === 0 || i === node.table.widths.length) ? 2 : 1;
          },
          hLineColor: function (i, node) {
            return 'black';
          },
          vLineColor: function (i, node) {
            return 'black';
          },
          hLineStyle: function (i, node) {
            if (i === 0 || i === node.table.body.length) {
              return null;
            }
            return {dash: {length: 10, space: 4}};
          },
          vLineStyle: function (i, node) {
            if (i === 0 || i === node.table.widths.length) {
              return null;
            }
            return {dash: {length: 4}};
          },
        },
      },

      styles: {
        header: {
          fontSize: 22,
          bold: true,
          margin: [0, 0, 0, 10]
        },
        subheader: {
          fontSize: 18,
          bold: true,
          margin: [0, 10, 0, 5]
        },
        tableExample: {
          margin: [0, 5, 0, 15]
        },
        tableHeader: {
          bold: true,
          fontSize: 12,
          color: 'black',
          alignment: 'center',
          fillColor: '#eeeeff'
        },
        tableRow: {
          bold: true,
          fontSize: 12,
          color: 'black',
          alignment: 'center',
          fillColor: '#eeffee'
        },
        tablebody: {
          fontSize: 11,
        },
        tableResult: {
          bold: true,
          fontSize: 13,
          color: 'black',
          alignment: 'justify',
          fillColor: '#d9e8f0'
        }

      },
      defaultStyle: {
         alignment: 'justify'
      }
    };

    pdfMake.createPdf(docDefinition).open();
  }

  private exportarCSV(): void {

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
