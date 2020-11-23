import { Component, OnInit, ViewChild } from '@angular/core';
import { MatDialogRef } from '@angular/material/dialog';
import { faTimesCircle, faChevronRight } from '@fortawesome/free-solid-svg-icons';
import { FormGroup, FormBuilder, Validators } from '@angular/forms';
import { SwalComponent } from '@sweetalert2/ngx-sweetalert2';
import { IdentityService, IRecoverPasswordCommand } from '../../../../domain/services/identity.service';
import { NgxSpinnerService } from 'ngx-spinner';
import { SweetAlertGenericMessageService } from '../../../../services/sweet-alert-generic-message.service';

@Component({
  selector: 'app-password-forgot',
  templateUrl: './password-forgot.component.html',
  styleUrls: ['./password-forgot.component.scss']
})
export class PasswordForgotComponent implements OnInit {

  @ViewChild('forgotPasswordValid') private swalFormValid: SwalComponent;

  // Font Awseome icons
  public faClose = faTimesCircle;
  public faArrowRight = faChevronRight;

  // Forms
  formPasswordForgot: FormGroup;

  constructor(  private dialogRef: MatDialogRef<PasswordForgotComponent>,
                private fb: FormBuilder,
                private identityService: IdentityService,
                private spinnerService: NgxSpinnerService,
                private sweetAlertGenericMessageService: SweetAlertGenericMessageService) {

    dialogRef.disableClose = true;

  }

  ngOnInit(): void {

    this.formPasswordForgot = this.fb.group({

      email     : ['', [ Validators.required, Validators.pattern( '[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,3}$' )] ]

    });

  }

  public closeMatDialog(): void {

    this.dialogRef.close();

  }

  public sendResetPassword(): void {

    // Marcar datos inválidos
    if ( this.formPasswordForgot.invalid ) {
      return Object.values( this.formPasswordForgot.controls ).forEach( control => {

        // Si es un objeto
        if ( control instanceof FormGroup ) {

          Object.values( control.controls ).forEach( subControl => subControl.markAsTouched() );

        } else {

          control.markAsTouched(); // Marcar todos como tocadas

        }

      } );

    } else {


      this.spinnerService.show();
      setTimeout(() => {
        this.spinnerService.hide();
      }, 5000);

      const params: IRecoverPasswordCommand = this.formPasswordForgot.value;

      this.identityService.recoverPassword( params ).subscribe(

        (res: any) => {

          console.log('TEST > ', res);
          this.spinnerService.hide();
          this.swalFormValid.fire();

        },
        (err) => {

          if ( err.error.code === 'unauthorized' ) {
            this.sweetAlertGenericMessageService.showAlertError('No tiene los permisos suficientes', 'Error');
          } else {
            this.sweetAlertGenericMessageService.showAlertError('El correo ingresado no pertenece a usuario alguno.', 'Correro no válido');
          }
          console.error(err);
          this.spinnerService.hide();

        });

      // TODO: Registrar si sale todo bien para verificar el mail.
      // De estar todo bien, se muestra un mensaje de confirmación y se cierra la ventana.
      // this.swalFormValid.fire();

    }
  }


  // getters
  get emailNovalido(): boolean {
    return ( this.formPasswordForgot.get('email').invalid && this.formPasswordForgot.get('email').touched );
  }


}
