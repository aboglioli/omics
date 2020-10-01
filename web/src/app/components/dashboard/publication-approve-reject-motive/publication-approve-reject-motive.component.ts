import { Component, OnInit, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA} from '@angular/material/dialog';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { faTimesCircle } from '@fortawesome/free-solid-svg-icons';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';

interface DialogData {
  approve: boolean;
  publicationName: string;
}


@Component({
  selector: 'app-publication-approve-reject-motive',
  templateUrl: './publication-approve-reject-motive.component.html',
  styleUrls: ['./publication-approve-reject-motive.component.scss']
})
export class PublicationApproveRejectMotiveComponent implements OnInit {

  // Font Awseome icons
  public faClose = faTimesCircle;

  public title = '';
  public formReason: FormGroup;

  constructor(
    public dialogRef: MatDialogRef<PublicationApproveRejectMotiveComponent>,
    @Inject(MAT_DIALOG_DATA) public data: DialogData,
    private fb: FormBuilder,
    private sweetAlertGenericService: SweetAlertGenericMessageService
  ) { }

  ngOnInit(): void {

    this.title = ( this.data.approve ) ? 'Aprobar publicación' : 'Rechazar publicación';

    this.formReason = this.fb.group({

      reason: ['', [ Validators.required, Validators.minLength(5) ] ],

    });

  }

  public onClose( reason?: string ): void {
    this.dialogRef.close(reason);
  }

  public onSubmitMessage(): void {

    if ( this.formReason.invalid ) {

      this.sweetAlertGenericService.showAlertError( 'Debe enviar una motivo válido' );

      return Object.values( this.formReason.controls ).forEach( control => {

        // Si es un objeto
        if ( control instanceof FormGroup ) {

          Object.values( control.controls ).forEach( subControl => subControl.markAsTouched() );

        } else {

          control.markAsTouched(); // Marcar todos como tocadas

        }

      } );

    } else {

      this.onClose(this.formReason.get('reason').value);

    }

  }

  // Getters
  get reasonNovalido(): boolean {
    return ( this.formReason.get('reason').invalid && this.formReason.get('reason').touched );
  }


}
