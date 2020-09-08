import { Component, OnInit, Output, EventEmitter } from '@angular/core';
import { faBars, faBell } from '@fortawesome/free-solid-svg-icons';
import { Router } from '@angular/router';
import { MatDialog } from '@angular/material/dialog';
import { LoginRegisterComponent } from '../../components/login-register/login-register.component';
import { AuthService } from 'src/app/domain/services/auth.service';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';

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
  isAccessUserLogIn: boolean;  // Para habilitar algunas acciones según si esta el usuario logueado

  constructor(  private router: Router,
                private authService: AuthService,
                private dialog: MatDialog,
                private sweetAlertGenericService: SweetAlertGenericMessageService ) {


    this.subscribeAuthService();

  }

  ngOnInit(): void {

    this.isAccessUserLogIn = this.authService.isLoggedIn();

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

    // TODO: Agregar sistema de notificaciones como otro componente interno
    this.sweetAlertGenericService.showUnderConstrucction();

  }


  public toggleSideNavUserMenu(): void {

    this.clickSideNavUserToggle.emit();

  }

  //#endregion

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

      }

    } );

  }

  public logout(): void {

    this.authService.logout();
    this.router.navigate(['/home']);

  }

  //#endregion


}
