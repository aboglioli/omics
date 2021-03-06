import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { NgxSpinnerService } from 'ngx-spinner';
import { faUsers } from '@fortawesome/free-solid-svg-icons';
import { AuthService } from '../../domain/services/auth.service';
import Swal from 'sweetalert2';
import { AuthorService, IGetByIdResponse } from '../../domain/services/author.service';
import { IAuthor } from '../../domain/models/author';
import { DonacionComponent } from '../../components/donacion/donacion.component';
import { MatDialog } from '@angular/material/dialog';
import { IUser } from '../../domain/models/user';
import { can } from 'src/app/domain/models';

@Component({
  selector: 'app-perfil',
  templateUrl: './perfil.component.html',
  styleUrls: ['./perfil.component.scss']
})
export class PerfilComponent implements OnInit {

  public profileData: IAuthor;
  public profileAvatar: string;
  public profileFollowers = 0;
  public memberSince: Date;
  public isSameAsUser = false;
  public followed = false;
  public isAccessUserLogIn = false;

  public authUser: IUser;
  public can = can;

  // Font Awseome icons
  public faFollowers = faUsers;


  constructor(
    private authorService: AuthorService,
    private activateRoute: ActivatedRoute,
    private router: Router,
    private spinnerService: NgxSpinnerService,
    private authService: AuthService,
    private dialog: MatDialog,
  ) { }

  ngOnInit(): void {

    this.authService.getUser().subscribe((user) => {
      this.authUser = user;
    });

    this.getUserDataByParams();

    this.isAccessUserLogIn = this.authService.isLoggedIn();

  }

  private getUserDataByParams(): void {

    this.spinnerService.show();
    setTimeout(() => {
      this.spinnerService.hide();
    }, 20000); // 20 segundos de espera máxima TODO: Agregar mensaje de error de pasar mucho tiempo


    this.activateRoute.params.subscribe( params => {

      this.authorService.getById(params.id).subscribe(

        (data: IGetByIdResponse) => {

          this.profileData = data.author;
          this.profileFollowers = data.author.followers;
          this.followed = data.reader ? data.reader.followed : false;
          this.assignProfileAvatar( this.profileData );

          this.memberSince = new Date(this.profileData.created_at);

          this.isSameAsUser = ( this.authService.getIdUser() === this.profileData.id );

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

  private assignProfileAvatar( pData: IAuthor ): void {


    if ( pData.profile_image ) {
      this.profileAvatar = pData.profile_image;
    } else {
      this.profileAvatar = undefined;
    }

  }

  //#region Acciones de botones

  public selectDonar(): void {

    // TODO: Agregar pantalla de donación
    const dialogRef = this.dialog.open(
      DonacionComponent,
      {
        panelClass: 'margin-dialog',
        data: {
          userToDonate: this.profileData
        }
      }
    );

  }

  public selectSeguir(): void {

    this.authorService.follow(this.profileData.id).subscribe(
      res => {
        this.getUserDataByParams();
      },
      err => {
        console.log(err);
      }
    );

  }

  public selectUnfollow(): void {

    this.authorService.unfollow(this.profileData.id).subscribe(
      res => {
        this.getUserDataByParams();
      },
      err => {
        console.log(err);
      }
    );

  }


  //#endregion

}
