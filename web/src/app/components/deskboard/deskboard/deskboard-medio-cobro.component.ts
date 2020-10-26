import { Component, Inject, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { faTimesCircle, faSave } from '@fortawesome/free-solid-svg-icons';

interface DialogData {
  mailCobro: string;
}

@Component({
  selector: 'app-configurar-medio-cobro',
  templateUrl: './deskboard-medio-cobro.component.html',
  styleUrls: ['./deskboard-medio-cobro.component.scss']
})
export class DeskboardMedioCobroComponent implements OnInit {

  // Font Awseome icons
  public faClose = faTimesCircle;
  public faSave = faSave;

  formMedioCobro: FormGroup;


  constructor(
    @Inject(MAT_DIALOG_DATA) public data: DialogData,
    public dialogRef: MatDialogRef<DeskboardMedioCobroComponent>,
    private fb: FormBuilder,
  ) {
    dialogRef.disableClose = true;
  }

  ngOnInit(): void {

    this.buildForms();
    console.log(this.data)

  }

  private buildForms(): void {

    this.formMedioCobro = this.fb.group({

      correo     : [ this.data.mailCobro, [ Validators.required, Validators.pattern( '^[a-zA-Z0-9]+[a-zA-Z0-9_.+-]*@[a-zA-Z0-9]+[a-zA-Z0-9-]*\.[a-zA-Z0-9-.]+$' )] ],

    });


  }

  public closeMatDialog(): void {

    this.dialogRef.close();

  }

  public onSave(): void {

    this.dialogRef.close( this.formMedioCobro.get('correo').value );

  }

  // Getters
  get correoNoValido(): boolean {
    return ( this.formMedioCobro.get('correo').invalid && this.formMedioCobro.get('correo').touched );
  }

}
