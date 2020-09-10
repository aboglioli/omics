import { Component, OnInit } from '@angular/core';
import { NgxSpinnerService } from 'ngx-spinner';
import { AuthService } from 'src/app/domain/services/auth.service';
import { IUser } from '../../domain/models/user';
import { IdentityService } from '../../domain/services/identity.service';
import { FormGroup, FormBuilder, Validators } from '@angular/forms';

@Component({
  selector: 'app-perfil-editar',
  templateUrl: './perfil-editar.component.html',
  styleUrls: ['./perfil-editar.component.scss']
})
export class PerfilEditarComponent implements OnInit {

  private userId: string;
  public userData: IUser;

  public formProfile: FormGroup;

  constructor(

    private spinnerService: NgxSpinnerService,
    private authService: AuthService,
    private identityService: IdentityService,
    private fb: FormBuilder,

  ) {}

  ngOnInit(): void {

    this.createForm();
    this.userId = this.authService.getIdUser();


    this.buildFormByIdentityService();

  }

  private createForm(): void  {

    this.formProfile = this.fb.group({

      username:       ['', [ Validators.required, Validators.minLength(5) ] ],
      email:          ['', [ Validators.pattern( '[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,3}$' )] ],
      name:           ['', ],
      lastname:       ['', ],
      birthdate:      ['', ],
      gender:         ['', ],
      profile_image:  ['', ],
      biography:      ['', ]

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

  // #region Getters
  get isProfileAvatarImageExists(): boolean {

    return ( this.profileAvatarImage ) ? true : false;

  }
  get profileAvatarImage(): string {

    return this.formProfile.get( 'profile_image' ).value;

  }

  // #endregion

}
