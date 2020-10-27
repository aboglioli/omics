import { Component, OnInit, ViewChild, ElementRef } from '@angular/core';
// import { jsPDF } from 'jspdf';
// import { html2canvas } from 'html2canvas';
import pdfMake from 'pdfmake';

import { ReportService } from '../../../domain/services/report.service';
import { IReport } from '../../../domain/models/report';

@Component({
  selector: 'dev-reports',
  templateUrl: './reports.html',
  styleUrls: ['../general.scss'],
})
export class DevReportsComponent implements OnInit {
  public report: IReport;

  private dateFrom: Date;
  private dateTo: Date;

  @ViewChild('pdfData', {static: false}) pdfData: ElementRef;

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
    const docDefinition = {
      content: [
        'Bulleted list example:',
        {
          ul: [
            'Item 1',
            'Item 2',
            'Item 3',
            { text: 'Item 4', bold: true },
          ]
        },

        'Numbered list example:',
        {
          ol: [
            'Item 1',
            'Item 2',
            'Item 3'
          ]
        }
      ]
    };

    pdfMake.fonts = {
       Roboto: {
         normal: 'https://cdnjs.cloudflare.com/ajax/libs/pdfmake/0.1.66/fonts/Roboto/Roboto-Regular.ttf',
         bold: 'https://cdnjs.cloudflare.com/ajax/libs/pdfmake/0.1.66/fonts/Roboto/Roboto-Medium.ttf',
         italics: 'https://cdnjs.cloudflare.com/ajax/libs/pdfmake/0.1.66/fonts/Roboto/Roboto-Italic.ttf',
         bolditalics: 'https://cdnjs.cloudflare.com/ajax/libs/pdfmake/0.1.66/fonts/Roboto/Roboto-MediumItalic.ttf'
       },
    }

    pdfMake.createPdf(docDefinition).open();

    // const mapItem = (obj) => {
    //   return Object.keys(obj)
    //     .map((key) => `- ${key}: ${obj[key]}.\n`)
    // };

    // doc.text(`
    //   Reporte de Omics. Desde ${this.dateFrom.toISOString()}, hasta ${this.dateTo.toISOString()}
    //
    //   USUARIOS (${this.report.users.total})
    //   · Por estado:
    //   ${mapItem(this.report.users.by_status)}
    //   · Por género:
    //   ${mapItem(this.report.users.by_gender)}
    //   · Por edad:
    //   ${mapItem(this.report.users.by_age)}
    //
    //   PUBLICACIONES (${this.report.publications.total})
    //   · Por categoría:
    //   ${mapItem(this.report.publications.by_category)}
    //   · Por contrato:
    //   ${mapItem(this.report.publications.by_contract)}
    //   · Por estado:
    //   ${mapItem(this.report.publications.by_status)}
    //   · Por cantidad de páginas:
    //   ${mapItem(this.report.publications.by_pages)}
    //
    //   SUSCRIPCIONES (${this.report.subscriptions.total})
    //   · Por estado:
    //   ${mapItem(this.report.subscriptions.by_status)}
    //
    //   CONTRATOS (${this.report.contracts.total})
    //   · Por estado:
    //   ${mapItem(this.report.contracts.by_status)}
    //   · Por monto cobrado:
    //   ${mapItem(this.report.contracts.by_amount)}
    //   · Por cantidad de pagos:
    //   ${mapItem(this.report.contracts.by_payment)}
    //
    //   DONACIONES (${this.report.donations.total})
    //   · Por estado:
    //   ${mapItem(this.report.donations.by_status)}
    //   · Por monto:
    //   ${mapItem(this.report.donations.by_amount)}
    //
    //   PAGOS
    //   · Ingresos: \$${this.report.payments.income.toFixed(2)}
    //   · Egresos: \$${this.report.payments.outcome.toFixed(2)}
    // `, 5, 5);

  }
}
