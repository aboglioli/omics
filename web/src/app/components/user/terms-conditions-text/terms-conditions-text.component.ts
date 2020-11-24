import { Component, OnInit } from '@angular/core';
import { MatDialogRef } from '@angular/material/dialog';
import { faTimesCircle } from '@fortawesome/free-solid-svg-icons';

@Component({
  selector: 'app-terms-conditions-text',
  templateUrl: './terms-conditions-text.component.html',
  styleUrls: ['./terms-conditions-text.component.scss']
})
export class TermsConditionsTextComponent implements OnInit {

  // FontAwesome Icon
  public faClose = faTimesCircle;

  public dialogRef: MatDialogRef<TermsConditionsTextComponent>;

  constructor() { }

  ngOnInit(): void {
  }

  public onClose(): void {
    this.dialogRef.close();
  }

}
