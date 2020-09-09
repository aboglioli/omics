import { Component, OnInit } from '@angular/core';
import { IdentityService } from '../../domain/services/identity.service';
import { ActivatedRoute, Router } from '@angular/router';
import { NgxSpinnerService } from 'ngx-spinner';
import { IUser } from '../../domain/models/user';
import { faUsers } from '@fortawesome/free-solid-svg-icons';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';
import { AuthService } from '../../domain/services/auth.service';
import Swal from 'sweetalert2';

@Component({
  selector: 'app-perfil',
  templateUrl: './perfil.component.html',
  styleUrls: ['./perfil.component.scss']
})
export class PerfilComponent implements OnInit {

  public profileData: IUser;
  public profileAvatar: string;
  public profileFollowers = 0;
  public memberSince: Date;
  public isSameAsUser = false;

  // Font Awseome icons
  public faFollowers = faUsers;


  constructor(
    private identifyService: IdentityService,
    private route: ActivatedRoute,
    private router: Router,
    private spinnerService: NgxSpinnerService,
    private authService: AuthService,
    private sweetAlertGenericService: SweetAlertGenericMessageService,
  ) { }

  ngOnInit(): void {

    this.getUserDataByParams();

  }

  private getUserDataByParams(): void {

    this.spinnerService.show();
    setTimeout(() => {
      this.spinnerService.hide();
    }, 20000); // 20 segundos de espera máxima TODO: Agregar mensaje de error de pasar mucho tiempo


    this.route.params.subscribe( params => {

      this.identifyService.getById(params.id).subscribe(

        (data: IUser) => {

          console.log(data);
          this.profileData = data;

          this.assignProfileAvatar( this.profileData );

          this.memberSince = new Date();

          this.isSameAsUser = ( this.authService.getIdUser() === this.profileData.id )

          this.spinnerService.hide();

        },

        (err: Error) =>  {

          console.error(err);
          Swal.fire({
            icon: 'error',
            title: 'Error',
            text: `El perfil ${ params.id } no existe.`,
            focusConfirm: true,
          }).then( result => {

            this.router.navigate(['/home']);

          } );

          this.spinnerService.hide();

        }

        );

    } );

  }

  private assignProfileAvatar( pData: IUser ): void {


    if ( pData.profile_image ) {
      this.profileAvatar = pData.profile_image;
    } else {
      this.profileAvatar = undefined;
    }

  }

  //#region Acciones de botones

  public selectDonar(): void {

    // TODO: Agregar pantalla de donación
    this.sweetAlertGenericService.showUnderConstrucction();

  }

  public selectSeguir(): void {

    // TODO: Agregar funcionalidad seguir
    this.sweetAlertGenericService.showUnderConstrucction();

  }


  //#endregion

}
