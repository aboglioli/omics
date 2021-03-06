import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-development',
  template: `
    <div class="menu">
      <button (click)="select('upload-file')">Upload file</button>
      <button (click)="select('new-publication')">New publication</button>
      <button (click)="select('payment')">Payment</button>
    </div>

    <div [ngSwitch]="selection">
      <dev-upload-file *ngSwitchCase="'upload-file'"></dev-upload-file>
      <dev-new-publication *ngSwitchCase="'new-publication'"></dev-new-publication>
      <dev-payment *ngSwitchCase="'payment'"></dev-payment>
      <dev-reports *ngSwitchCase="'reports'"></dev-reports>
    </div>
  `,
  styleUrls: ['./general.scss'],
})
export class DevelopmentComponent implements OnInit {
  public selection = 'reports';

  constructor(
  ) { }

  ngOnInit(): void {}

  select(selection: string): void {
    this.selection = selection;
  }
}
