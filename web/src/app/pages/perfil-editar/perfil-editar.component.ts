import { Component, OnInit } from '@angular/core';
import { NgxSpinnerService } from 'ngx-spinner';
import { AuthService } from 'src/app/domain/services/auth.service';
import { IUser } from '../../domain/models/user';
import { faEdit } from '@fortawesome/free-solid-svg-icons';
import { IdentityService } from '../../domain/services/identity.service';
import { FormGroup, FormBuilder, Validators } from '@angular/forms';
import { FileService } from 'src/app/domain/services/file.service';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';

@Component({
  selector: 'app-perfil-editar',
  templateUrl: './perfil-editar.component.html',
  styleUrls: ['./perfil-editar.component.scss']
})
export class PerfilEditarComponent implements OnInit {

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
    private sweetAlertGenericService: SweetAlertGenericMessageService,

  ) {}

  ngOnInit(): void {

    this.maxDateBirthday = new Date();

    this.createForm();
    this.userId = this.authService.getIdUser();

    this.buildFormByIdentityService();

  }

  private createForm(): void  {

    this.formProfile = this.fb.group({

      username:       ['', [ Validators.required, Validators.minLength(5) ] ],
      email:          ['', [ Validators.required, Validators.pattern( '^[a-zA-Z0-9]+[a-zA-Z0-9_.+-]*@[a-zA-Z0-9]+[a-zA-Z0-9-]*\.[a-zA-Z0-9-.]+$' )] ],
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

      username: userDataObject.username,
      email: userDataObject.email,
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
    return this.formProfile.get('biography').value.length;
  }

  // #endregion

}
