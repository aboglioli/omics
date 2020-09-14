import { Component, OnInit, Inject, ViewChild } from '@angular/core';
import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material/dialog';
import { faTimesCircle, faChevronCircleRight, faEyeSlash, faEye } from '@fortawesome/free-solid-svg-icons';
import { FormGroup, FormBuilder, Validators } from '@angular/forms';
import { ValidadoresCustomService } from 'src/app/services/validadores-custom.service';
import { NgxSpinnerService } from 'ngx-spinner';
import { IdentityService } from 'src/app/domain/services/identity.service';
import { IChangePasswordCommand } from '../../../domain/services/identity.service';
import { SwalComponent } from '@sweetalert2/ngx-sweetalert2';
import { Router } from '@angular/router';

@Component({
  selector: 'app-password-rewrite',
  templateUrl: './password-rewrite.component.html',
  styleUrls: ['./password-rewrite.component.scss']
})
export class PasswordRewriteComponent implements OnInit {


  @ViewChild('formUserNotValid') private swalFormUserNotValid: SwalComponent;
  @ViewChild('formUserSameOldNewPass') private swalFormUserSameOldNewPass: SwalComponent;
  @ViewChild('formUserValid') private swalFormUserValid: SwalComponent;


  // Font Awseome icons
  public faClose = faTimesCircle;
  public faArrowRight = faChevronCircleRight;
  public faEyeOpen = faEye;
  public faEyeSlash = faEyeSlash;
  public isRecoveryPassword = true;

  // Forms
  formRewritePassword: FormGroup;

  // Otros
  passwordProperties =  {
    type: 'password',
    visible: false
  };

  constructor(  @Inject(MAT_DIALOG_DATA) public data: any,
                private dialogRef: MatDialogRef<PasswordRewriteComponent>,
                private validadoresCustom: ValidadoresCustomService,
                private fb: FormBuilder,
                private spinnerService: NgxSpinnerService,
                private identityService: IdentityService,
                private router: Router) {


    dialogRef.disableClose = true;

  }

  ngOnInit(): void {

    this.isRecoveryPassword = this.data.isRecoveryPassword;
    this.formCreate();

  }

  private formCreate(): void {

    this.formRewritePassword = this.fb.group({

      oldPassword: [this.data.temporalPass, [ Validators.required, Validators.minLength(8) ]],
      password1  : ['', [ Validators.required, Validators.minLength(8) ] ],
      password2  : ['', [ Validators.required, Validators.minLength(8) ] ],

    }, {
      // A nivel formulario asyncValidators (sino uno creado)
      validators: this.validadoresCustom.passwordsIguales('password1', 'password2') // Validador personal asyncrono
    });

  }

  public sendRewritePassword(): void {

    // Marcar datos inválidos
    if ( this.formRewritePassword.invalid ) {

      return Object.values( this.formRewritePassword.controls ).forEach( control => {

        // Si es un objeto
        if ( control instanceof FormGroup ) {

          Object.values( control.controls ).forEach( subControl => subControl.markAsTouched() );

        } else {

          control.markAsTouched(); // Marcar todos como tocadas

        }

      } );

    } else {

      this.spinnerService.show();

      const passwordChange: IChangePasswordCommand = {
        old_password: this.formRewritePassword.get('oldPassword').value,
        new_password: this.formRewritePassword.get('password1').value
      };


      this.identityService.changePassword( this.data.userId, passwordChange ).subscribe(

        (res: any) => {

          console.log(res);
          this.spinnerService.hide();
          this.swalFormUserValid.fire();


        }, (err: any ) => {

          this.spinnerService.hide();
          console.error(err);

          const errorCode = err.error.code;

          if ( errorCode === 'are_the_same' ) {
            this.swalFormUserSameOldNewPass.fire();
          } else {

            this.swalFormUserNotValid.fire();

          }

        }
      );



    }

  }

  public closeMatDialog( isValidClose: boolean ): void {

    if ( isValidClose ) {

      this.dialogRef.close();

    } else {

      if (  this.isRecoveryPassword ) {
        this.router.navigate(['/home']);
        this.dialogRef.close();
      }

    }

  }

  public showPassword( show: boolean ): void {

    const newValueVisible = !show;

    this.passwordProperties.visible = newValueVisible;
    this.passwordProperties.type = ( newValueVisible ) ? 'text' : 'password';

  }

  // getters
  get oldPasswordChangeNoValido(): boolean {
    return ( this.formRewritePassword.get('oldPassword').invalid && this.formRewritePassword.get('oldPassword').touched );
  }

  get password1ChangeNoValido(): boolean {
    return ( this.formRewritePassword.get('password1').invalid && this.formRewritePassword.get('password1').touched );
  }

  get password2ChangeNoValido(): boolean {

    const pass1 = this.formRewritePassword.get('password1').value;
    const pass2 = this.formRewritePassword.get('password2').value;

    return ( pass1 === pass2 ) ? false : true;

  }


}
