import { Component, OnInit, ViewChild } from '@angular/core';
import { NgxSpinnerService } from 'ngx-spinner';
import { AuthService } from 'src/app/domain/services/auth.service';
import { IUser } from '../../domain/models/user';
import { faEdit } from '@fortawesome/free-solid-svg-icons';
import { IdentityService, IUpdateCommandUser } from '../../domain/services/identity.service';
import { FormGroup, FormBuilder, Validators } from '@angular/forms';
import { FileService } from 'src/app/domain/services/file.service';
import { MatDialog } from '@angular/material/dialog';
import { PasswordRewriteComponent } from 'src/app/components/password-recovery/password-rewrite/password-rewrite.component';
import { SwalComponent } from '@sweetalert2/ngx-sweetalert2';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';
import Swal from 'sweetalert2';
import { Router } from '@angular/router';

@Component({
  selector: 'app-perfil-editar',
  templateUrl: './perfil-editar.component.html',
  styleUrls: ['./perfil-editar.component.scss']
})
export class PerfilEditarComponent implements OnInit {

  @ViewChild('formDataInvalid') private swalFormDataInvalid: SwalComponent;

  // FontAwesome Icon
  public faEditField = faEdit;

  // Usuario
  private userId: string;
  public userData: IUser;

  // Del formulario
  public formProfile: FormGroup;
  public maxDateBirthday: Date;

  public generosList  = [
    {
      name: 'Femenino',
      value: 'female'
    },
    {
      name: 'Masculino',
      value: 'male'
    },
    {
      name: 'Otro',
      value: 'other'
    }
  ];

  constructor(

    private spinnerService: NgxSpinnerService,
    private authService: AuthService,
    private fileServ: FileService,
    private identityService: IdentityService,
    private fb: FormBuilder,
    private router: Router,
    private dialog: MatDialog,
    private sweetAlertGenericService: SweetAlertGenericMessageService

  ) {}

  ngOnInit(): void {

    this.maxDateBirthday = new Date();

    this.createForm();
    this.userId = this.authService.getIdUser();

    this.buildFormByIdentityService();

  }

  private createForm(): void  {

    // TODO: corroborar el cambio de email y username que IUpdateCommand no lo acepta
    this.formProfile = this.fb.group({

      username:       [ { value: '', disabled: true }, [ Validators.required, Validators.minLength(5) ] ],
      email:          [ { value: '', disabled: true }, [ Validators.required, Validators.pattern( '^[a-zA-Z0-9]+[a-zA-Z0-9_.+-]*@[a-zA-Z0-9]+[a-zA-Z0-9-]*\.[a-zA-Z0-9-.]+$' )] ],
      name:           ['', [ Validators.required, Validators.minLength(2) ]],
      lastname:       ['', [ Validators.required, Validators.minLength(2) ]],
      birthdate:      ['', ],
      gender:         ['', ],
      profile_image:  ['', ],
      biography:      ['', [Validators.maxLength(252) ] ],
    });

  }

  private buildFormByIdentityService(): void {

    this.spinnerService.show();

    this.identityService.getById(this.userId).subscribe(
      (resData: IUser) => {

        this.userData = resData;
        this.buildFormByObject(this.userData);
        this.spinnerService.hide();

      },
      (err: Error) => {

        console.error(err);
        this.spinnerService.hide();

      }
    );

  }

  private buildFormByObject( userDataObject: IUser ): void {

    this.formProfile.reset({ // Para actualizar algunos se utiliza patchValue

      username: { value: userDataObject.username, disabled: true },
      email: { value: userDataObject.email, disabled: true },
      name: userDataObject.name,
      lastname: userDataObject.lastname,
      birthdate: userDataObject.birthdate,
      gender: userDataObject.gender,
      profile_image: userDataObject.profile_image,
      biography: userDataObject.biography,

    });

    console.log(this.formProfile.value);

  }

  public uploadImageAvatar(): void {

    // Crear elemento input de tipo 'file' para poder manejarlo desde el botón que lo llama
    const inputFileElement = document.createElement('input');
    inputFileElement.type = 'file'; // Nota:  Solo uno a la vez, para varios: inputFileElement.multiple = multiple
    inputFileElement.accept = '.png, .jpg, .jpeg';
    inputFileElement.click();


    // Definir la función del llamado al hacer click (cuando realiza un cambio)
    inputFileElement.onchange = ( event: any ) => {

      const fdImage: FormData = new FormData();
      const imagenAvatar  = event.target.files[0];

      this.spinnerService.show();
      fdImage.append('image', imagenAvatar, imagenAvatar.name);

      this.fileServ.upload(fdImage).subscribe(
        (res: any) => {

          this.formProfile.get('profile_image').setValue( res.files[0].url);

          this.spinnerService.hide();

        }, (err: Error) => {

          // TODO: Manejar error por si se cae S3
          console.error(err);
          this.spinnerService.hide();

        }
      );

    };

  }

  public convertDateToRFC3339(changeDate: Date): void {

    this.formProfile.get('birthdate').setValue( changeDate.toISOString() );

  }

  public onGuardarCambios(): void {

    console.log(this.formProfile.value);

    if ( this.formProfile.invalid ) {

      this.swalFormDataInvalid.fire();

      return Object.values( this.formProfile.controls ).forEach( control => {

        // Si es un objeto
        if ( control instanceof FormGroup ) {

          Object.values( control.controls ).forEach( subControl => subControl.markAsTouched() );

        } else {

          control.markAsTouched(); // Marcar todos como tocadas

        }

      } );

    } else {

      this.spinnerService.show();
      const userNewData: IUpdateCommandUser = this.formProfile.value;

      this.identityService.update( this.userId, userNewData ).subscribe(

        (res: any) => {

          Swal.fire({
            icon: 'success',
            title: 'Actualizado',
            text: 'Los datos fueron actualizados exitosamente'
          }).then(  (result) => {
    
            this.router.navigate([`/profile/${ this.userId}`]);
      
          });

          
          this.spinnerService.hide();

        },
        (err: Error) => {

          console.error(err);
          this.spinnerService.hide();

        }

      );

    }

  }

  public onSuscribirse(): void {

    // TODO: Agregar pantalla de suscripción
    this.sweetAlertGenericService.showUnderConstrucction();

  }

  public onMedioCobro(): void {

    // TODO: Agregar pantalla de Medio de Cobro
    this.sweetAlertGenericService.showUnderConstrucction();

  }

  public onEliminarCuenta(): void {

    // TODO: Agregar pantalla de eliminar
    this.sweetAlertGenericService.showUnderConstrucction();

  }

  public onChangePassword(): void {

    const dialogRef = this.dialog.open(PasswordRewriteComponent, {
      data: {
              userId:  this.userId,
              isRecoveryPassword: false
            },
      panelClass: 'no-padding-dialog'
    });

  }

  // #region Getters
  get isProfileAvatarImageExists(): boolean {

    return ( this.profileAvatarImage ) ? true : false;

  }
  get profileAvatarImage(): string {
    return this.formProfile.get( 'profile_image' ).value;
  }

  get nombreNovalido(): boolean {
    return ( this.formProfile.get('name').invalid && this.formProfile.get('name').touched );
  }

  get apellidoNovalido(): boolean {
    return ( this.formProfile.get('lastname').invalid && this.formProfile.get('lastname').touched );
  }

  get correoNovalido(): boolean {
    return ( this.formProfile.get('email').invalid && this.formProfile.get('email').touched );
  }

  get biografiaNovalido(): boolean {
    return ( this.formProfile.get('biography').invalid && this.formProfile.get('biography').touched );
  }

  get biografiaLenght(): number {

    const bio = this.formProfile.get('biography').value;

    return ( bio ) ? bio.length : 0;

  }

  // #endregion

}
