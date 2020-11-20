import { Component, OnInit, ViewChild } from '@angular/core';
import { faTimesCircle, faChevronCircleRight, faChevronCircleDown, faEnvelopeSquare, faEye, faEyeSlash } from '@fortawesome/free-solid-svg-icons';
import { FormGroup, FormBuilder, Validators } from '@angular/forms';
import { MatDialogRef, MatDialog } from '@angular/material/dialog';
import { ValidadoresCustomService } from '../../../services/validadores-custom.service';
import { Router } from '@angular/router';
import {  IdentityService, IRegisterCommand, IRegisterResponse,
          ILoginCommand, ILoginResponse } from '../../../domain/services/identity.service';
import { AuthService } from 'src/app/domain/services/auth.service';
import { PasswordForgotComponent } from '../password-recovery/password-forgot/password-forgot.component';
import { NgxSpinnerService } from 'ngx-spinner';
import { SwalComponent } from '@sweetalert2/ngx-sweetalert2';
import Swal from 'sweetalert2';
import { IUser } from 'src/app/domain/models';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';

@Component({
  selector: 'app-login-register',
  templateUrl: './login-register.component.html',
  styleUrls: ['./login-register.component.scss']
})
export class LoginRegisterComponent implements OnInit {

  @ViewChild('formLoginInvalid') private swalFormLoginInvalid: SwalComponent;
  @ViewChild('formSignUpValid') private swalFormSignUpValid: SwalComponent;

  // Font Awseome icons
  public faClose = faTimesCircle;
  public faSubmit = faChevronCircleRight;
  public faEmail = faEnvelopeSquare;
  public faSignUp = faChevronCircleDown;
  public faEyeOpen = faEye;
  public faEyeSlash = faEyeSlash;

  // Forms
  formLogin: FormGroup;
  formSignUp: FormGroup;

  maxDatebirthdate = new Date();

  // Otros atributos
  isLoginOptionShow = true;
  userData: IUser;

  passwordListType = {

    login: {

      type: 'password',
      visible: false

    },
    signUpFirst: {

      type: 'password',
      visible: false

    },
    signUpSecond: {

      type: 'password',
      visible: false

    },

  };

  constructor(  private dialogRef: MatDialogRef<LoginRegisterComponent>,
                private fb: FormBuilder,
                private validadoresCustom: ValidadoresCustomService,
                private router: Router,
                private identityService: IdentityService,
                private authService: AuthService,
                private dialog: MatDialog,
                private spinnerService: NgxSpinnerService,
                private sweetAlertGenericService: SweetAlertGenericMessageService,
                ) {

    dialogRef.disableClose = true;

  }

  ngOnInit(): void {

    this.maxDatebirthdate.setFullYear(  this.maxDatebirthdate.getFullYear() - 14 );
    this.buildForms();


  }

  public closeMatDialog(): void {

    this.dialogRef.close();

  }

  private buildForms(): void {

    // Login
    this.formLogin = this.fb.group({

      correoUsuario    : ['', [ Validators.required, Validators.minLength(5) ] ],
      password         : ['', [ Validators.required, Validators.minLength(8) ] ],

    });

    // SignUp
    this.formSignUp = this.fb.group({

      correo     : ['', [ Validators.required, Validators.pattern( '^[a-zA-Z0-9]+[a-zA-Z0-9_.+-]*@[a-zA-Z0-9]+[a-zA-Z0-9-]*\.[a-zA-Z0-9-.]+$' )] ],
      usuario    : ['', [ Validators.required, Validators.minLength(5) ]],
      birthdate  : ['', [ Validators.required ] ],
      password1  : ['', [ Validators.required, Validators.minLength(8) ] ],
      password2  : ['', [ Validators.required, Validators.minLength(8) ] ],

    }, {
      // A nivel formulario asyncValidators (sino uno creado)
      validators: this.validadoresCustom.passwordsIguales('password1', 'password2') // Validador personal asyncrono
    });

  }

  public loginUser(): void {

    // Marcar datos inválidos
    if ( this.formLogin.invalid ) {

      return Object.values( this.formLogin.controls ).forEach( control => {

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
      }, 10000);

      const loginCommand: ILoginCommand = {

        username: this.formLogin.get('correoUsuario').value,
        password: this.formLogin.get('password').value

      };

      this.identityService.login(loginCommand).subscribe(

        (res: ILoginResponse) => {

          this.authService.setToken( res.auth_token, res.user_id );
          this.router.navigate(['/home']);

          this.setUserData( this.authService.getIdUser() );

          this.spinnerService.hide();

        },
        (error: any) => {
          console.error( 'ERROR !!!', error );

          this.swalFormLoginInvalid.fire();
          this.spinnerService.hide();

        }
      );

    }

  }

  private setUserData( uderId: string ): void {

    this.identityService.getById( uderId ).subscribe( res => {

      this.userData = res;

      Swal.fire({
          icon: 'success',
          title: '¡BIENVENIDO!',
          text: `Bienvenido ${ this.userData.username } a Omics.`,
          focusConfirm: true,
      }).then( result => {

        this.closeMatDialog();

      } );



    } );

  }

  public signUpUser(): void {

    // Quitar espacios principio y final de más
    const userNameAux = ( this.formSignUp.get('usuario').value.trim() );
    this.formSignUp.get('usuario').setValue( userNameAux );

    const correoAux = ( this.formSignUp.get('correo').value.trim() );
    this.formSignUp.get('correo').setValue( correoAux );

    if ( userNameAux.indexOf(' ') > 0) {

      this.sweetAlertGenericService.showAlertError('Los nombres de usuario no pueden tener espacios', 'Error nombre de usuario');

    } else {

      // Marcar datos inválidos
      if ( this.formSignUp.invalid ) {

        return Object.values( this.formSignUp.controls ).forEach( control => {

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
        }, 10000);


        const registerCommand: IRegisterCommand = {

          username: this.formSignUp.get('usuario').value,
          email: this.formSignUp.get('correo').value,
          password: this.formSignUp.get('password1').value,
          birthdate: this.formSignUp.get('birthdate').value

        };


        this.identityService.register( registerCommand ).subscribe(

          (result: IRegisterResponse) => {

            // TODO: En vez de señalar que se esconda luego el popup, habria que hacer se que cambie a la pestaña de Login
            // console.log('TEST > Registro realizado con éxito', result);

            this.router.navigate(['/home']);

            this.swalFormSignUpValid.fire();
            this.spinnerService.hide();

          },
          (error: any ) => {

            console.error( 'ERROR !!!', error );

            const errorContext = error.error.context;

            /* Se comprueba que campos de contexto hay del error, se muestra el mas prioritario
            (buena práctica no colocar todo lo que esta erroneo) */
            let msgAux: string;

            if ( errorContext.email ) {
              msgAux = 'correo';
            } else {

              if ( errorContext.username ) {
                msgAux = 'usuario';
              }

            }

            Swal.fire({
              icon: 'error',
              title: 'Error al registrarse',
              text: `El ${ msgAux } no está disponible`
            });

            this.spinnerService.hide();

          }
        );

      }

    }

  }

  public changeLoginShow( indexTab: number ): void {

    this.isLoginOptionShow = (indexTab === 1) ? false : true;

  }

  public submitForm(): void {

    if ( this.isLoginOptionShow ) {

      this.loginUser();

    } else {

      this.signUpUser();

    }


  }

  // Extra password
  public showPasswordLogin( passwordType: string, show: boolean ): void {

    const newValueVisible = !show;

    this.passwordListType[passwordType].visible = newValueVisible;
    this.passwordListType[passwordType].type = ( newValueVisible ) ? 'text' : 'password';

  }

  public openForgetPassword(): void {

    const dialogRef = this.dialog.open(PasswordForgotComponent, {panelClass: 'no-padding-dialog'} );

  }

  public convertDateToRFC3339(changeDate: Date): void {

    this.formSignUp.get('birthdate').setValue( changeDate.toISOString() );

  }

  // #region getters
  get correoUsuarioLoginNovalido(): boolean {
    return ( this.formLogin.get('correoUsuario').invalid && this.formLogin.get('correoUsuario').touched );
  }

  get passwordLoginNoValido(): boolean {
    return ( this.formLogin.get('password').invalid && this.formLogin.get('password').touched );
  }

  get correoSignUpNovalido(): boolean {
    return ( this.formSignUp.get('correo').invalid && this.formSignUp.get('correo').touched );
  }

  get usuarioNoValido(): boolean {
    return ( this.formSignUp.get('usuario').invalid && this.formSignUp.get('usuario').touched );
  }

  get password1SignUpNoValido(): boolean {
    return ( this.formSignUp.get('password1').invalid && this.formSignUp.get('password1').touched );
  }

  get password2SignUpNoValido(): boolean {

    const pass1 = this.formSignUp.get('password1').value;
    const pass2 = this.formSignUp.get('password2').value;

    return ( pass1 === pass2 ) ? false : true;

  }

  get fechaUsuarioSignUpNovalido(): boolean {
    return ( this.formSignUp.get('birthdate').invalid && this.formSignUp.get('birthdate').touched );
  }

  // #endregion

}
