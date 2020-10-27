import { Component, OnInit, ViewChild, ElementRef } from '@angular/core';
import pdfMake from 'pdfmake';

import { ReportService } from '../../../domain/services/report.service';
import { IReport } from '../../../domain/models/report';

pdfMake.fonts = {
   Roboto: {
     normal: 'https://cdnjs.cloudflare.com/ajax/libs/pdfmake/0.1.66/fonts/Roboto/Roboto-Regular.ttf',
     bold: 'https://cdnjs.cloudflare.com/ajax/libs/pdfmake/0.1.66/fonts/Roboto/Roboto-Medium.ttf',
     italics: 'https://cdnjs.cloudflare.com/ajax/libs/pdfmake/0.1.66/fonts/Roboto/Roboto-Italic.ttf',
     bolditalics: 'https://cdnjs.cloudflare.com/ajax/libs/pdfmake/0.1.66/fonts/Roboto/Roboto-MediumItalic.ttf'
   },
};

@Component({
  selector: 'dev-reports',
  templateUrl: './reports.html',
  styleUrls: ['../general.scss'],
})
export class DevReportsComponent implements OnInit {
  public report: IReport;

  private dateFrom: Date;
  private dateTo: Date;

  constructor(
    private reportService: ReportService,
  ) { }

  ngOnInit(): void {
    this.dateTo = new Date();
    this.dateFrom = new Date(this.dateTo);
    this.dateFrom.setHours(this.dateFrom.getHours() - 24 * 90);

    this.reportService.generate({
      date_from: this.dateFrom.toISOString(),
      date_to: this.dateTo.toISOString(),
    }).subscribe(
      (res) => {
        this.report = res;
      }
    );
  }

  downloadAsPDF() {
    if (!this.report) {
      return;
    }

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
        `Desde: ${this.dateFrom.toISOString()} - Hasta: ${this.dateTo.toISOString()}`,

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
}
