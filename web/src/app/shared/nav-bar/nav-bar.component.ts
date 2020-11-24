import { Component, OnInit, Output, EventEmitter } from '@angular/core';
import { Observable } from 'rxjs';
import { faBars, faBell } from '@fortawesome/free-solid-svg-icons';
import { Router } from '@angular/router';
import { MatDialog } from '@angular/material/dialog';
import { AuthService } from 'src/app/domain/services/auth.service';
import { IUser, can } from 'src/app/domain/models';
import { LoginRegisterComponent } from 'src/app/components/user/login-register/login-register.component';
import { NotificationService } from '../../domain/services/notification.service';

@Component({
  selector: 'app-nav-bar',
  templateUrl: './nav-bar.component.html',
  styleUrls: ['./nav-bar.component.scss']
})
export class NavBarComponent implements OnInit {

  @Output() clickSideNavMainToggle = new EventEmitter();
  @Output() clickSideNavUserToggle = new EventEmitter();

  // Font Awseome icons
  public faBars = faBars;
  public faBell = faBell;

  // Propios
  public isAccessUserLogIn: boolean;  // Para habilitar algunas acciones según si esta el usuario logueado
  public userData: IUser;
  public userAvatar: string;
  public user$: Observable<IUser>;
  public can = can;
  public notificationUnreadTotal: number = 0;

  constructor(  private router: Router,
                private authService: AuthService,
                private dialog: MatDialog,
                private notificactionService: NotificationService ) {


    this.subscribeAuthService();

  }

  ngOnInit(): void {

    this.isAccessUserLogIn = this.authService.isLoggedIn();

    if (  this.isAccessUserLogIn ) {
      const userId = this.authService.getIdUser();
      this.setAvatarImageFromUser( userId );
    }

    this.user$ = this.authService.getUser();

    // Obtener notificaciones
    // @TODO: No esta funcionando la llamada para obtener no leidos, asi que se aplica un filtro (tampoco se hace en tiempor real)
    this.notificactionService.getAll( {read: false} ).subscribe(
      (res) => {
        this.notificationUnreadTotal = res.notifications.filter(
          (notification) => {
            return !notification.read;
          }
        ).length;
        // console.log('TEST > ', res);
      }
    );


  }


  public toggleSideNavMainMenu(): void {

    this.clickSideNavMainToggle.emit();

  }

  public goToPage( pagePath: string ): void {

    this.router.navigate( ['/', pagePath] );

  }

  public openLoginRegisterDialog(): void {

    const dialogRef = this.dialog.open(LoginRegisterComponent);

  }


  //#region User Actions
  public showNotifications(): void {

    this.router.navigate(['/notifications']);

  }


  public toggleSideNavUserMenu(): void {

    this.clickSideNavUserToggle.emit();

  }

  //#endregion

  private setAvatarImageFromUser( idUser: string): void {

    this.authService.getUser().subscribe((user?: IUser) => {
      if (user) {
        this.userData = user;

        if ( this.userData.profile_image ) {
          this.userAvatar = this.userData.profile_image;
        } else {
          this.userAvatar = undefined;
        }
      }
    });

  }

  //#region  Auth User
  private subscribeAuthService(): void {

    // Para comprobar en tiempo real si tiene o no acceso el usuario
    this.authService.accessUser$.subscribe( (data: boolean) => {

      // Para actualizar si el usuario no esta más logueado
      if ( data === false && this.isAccessUserLogIn ) {

        this.isAccessUserLogIn = false;
        this.logout();

      } else {

        this.isAccessUserLogIn = data;

        if ( this.isAccessUserLogIn ){
          const userId = this.authService.getIdUser();
          this.setAvatarImageFromUser( userId );
        }


      }

    } );

  }


  public logout(): void {

    this.authService.logout();
    this.router.navigate(['/home']);

  }

  //#endregion


}
