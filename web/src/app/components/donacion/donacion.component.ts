import { Component, Inject, OnInit, OnDestroy } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { IAuthor } from 'src/app/domain/models';
import { faTimesCircle, faMoneyBillAlt, faDollarSign } from '@fortawesome/free-solid-svg-icons';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { BusinessRulesService } from 'src/app/domain/services/business-rules.service';
import { NgxSpinnerService } from 'ngx-spinner';
import { IBusinessRules } from '../../domain/models/business-rules';
import { AuthorService, IDonateResponse } from 'src/app/domain/services/author.service';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';

interface DialogData {
  userToDonate: IAuthor;
}

@Component({
  selector: 'app-donacion',
  templateUrl: './donacion.component.html',
  styleUrls: ['./donacion.component.scss']
})
export class DonacionComponent implements OnInit, OnDestroy {

  // Font Awseome icons
  public faClose = faTimesCircle;
  public faDonar = faMoneyBillAlt;
  public faCurrency = faDollarSign;

  userToDonate: IAuthor;
  formDonate: FormGroup;

  minValueDonate = 0;

  private lastInterval: number;

  constructor(
    @Inject(MAT_DIALOG_DATA) public data: DialogData,
    public dialogRef: MatDialogRef<DonacionComponent>,
    private fb: FormBuilder,
    private spinnerService: NgxSpinnerService,
    private businessRulesService: BusinessRulesService,
    private authorService: AuthorService,
    private sweetAlertGenericService: SweetAlertGenericMessageService,
  ) {
    dialogRef.disableClose = true;
  }

  ngOnInit(): void {

    this.userToDonate = this.data.userToDonate;
    this.buildForms();

    this.getMinValueDonate();

  }

  ngOnDestroy(): void {
    clearInterval(this.lastInterval);
  }

  private getMinValueDonate(): void  {

    this.spinnerService.show();
    this.businessRulesService.get().subscribe(

      (res: IBusinessRules) => {

        this.spinnerService.hide();
        this.minValueDonate = res.minimum_donation_amount;

        this.formDonate.get('monto').setValidators( [Validators.required, Validators.min(this.minValueDonate) ] );
        this.formDonate.get('monto').setValue(this.minValueDonate);

      },
      (err: Error ) => {

        this.spinnerService.hide();
        console.error('Error', err);

      }

    );

  }

  public closeMatDialog(): void {

    this.dialogRef.close();

  }

  private buildForms(): void {

    this.formDonate = this.fb.group({

      monto      :  [ 0, [ Validators.required, Validators.min(0) ] ],
      comentario :  [ '', [ Validators.max(256) ] ]

    });

  }

  public onDonate(): void {

    this.spinnerService.show();

    const monto = this.formDonate.get('monto').value;
    const comentario = this.formDonate.get('comentario').value;

    this.authorService.donate(this.userToDonate.id, { amount: monto, comment: comentario}).subscribe(
      (res: IDonateResponse) => {

        this.spinnerService.hide();

        this.sweetAlertGenericService.showAlertSuccess(`Donación de $${monto} realizada a ${ this.userToDonate.username }.`, 'Gracias por tu aporte')
        window.open( res.payment_link, '_blank' );

        this.closeMatDialog();

      },
      (err: Error ) => {

        this.spinnerService.hide();

        this.sweetAlertGenericService.showAlertError('Problemas al realizar la donación', 'Error');
        console.error('Error: ', err);

      }
    )

  }

  // Getters
  get montoNoValido(): boolean {
    return ( this.formDonate.get('monto').invalid && this.formDonate.get('monto').touched );
  }

  get comentarioNoValido(): boolean {
    return ( this.formDonate.get('comentario').invalid && this.formDonate.get('comentario').touched );
  }

  get commentLenght(): number {
    return this.formDonate.get('comentario').value.length;
  }

}
