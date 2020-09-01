import { Component, OnInit, ViewChild } from '@angular/core';
import { MatDialogRef } from '@angular/material/dialog';
import { faTimesCircle, faChevronRight } from '@fortawesome/free-solid-svg-icons';
import { FormGroup, FormBuilder, Validators } from '@angular/forms';
import { SwalComponent } from '@sweetalert2/ngx-sweetalert2';

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
                private fb: FormBuilder) {

    dialogRef.disableClose = true;

  }

  ngOnInit(): void {

    this.formPasswordForgot = this.fb.group({

      correo     : ['', [ Validators.required, Validators.pattern( '[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,3}$' )] ]

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

      // TODO: Registrar si sale todo bien para verificar el mail.
      // De estar todo bien, se muestra un mensaje de confirmación y se cierra la ventana.
      this.swalFormValid.fire();

    }
  }


  // getters
  get correoNovalido(): boolean {
    return ( this.formPasswordForgot.get('correo').invalid && this.formPasswordForgot.get('correo').touched );
  }


}
