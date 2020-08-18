import { Component, OnInit, Output, EventEmitter } from '@angular/core';
import { faBars } from '@fortawesome/free-solid-svg-icons';
import { Router } from '@angular/router';
import { MatDialog } from '@angular/material/dialog';
import { LoginRegisterComponent } from '../../components/login-register/login-register.component';
import { AuthService } from 'src/app/domain/services/auth';

@Component({
  selector: 'app-nav-bar',
  templateUrl: './nav-bar.component.html',
  styleUrls: ['./nav-bar.component.scss']
})
export class NavBarComponent implements OnInit {

  @Output() clickSideNavToggle = new EventEmitter();

  // Font Awseome icons
  public faBars = faBars;

  // Propios
  isAccessUserLogIn: boolean;  // Para habilitar algunas acciones según si esta el usuario logueado

  constructor(  private router: Router,
                private authService: AuthService,
                public dialog: MatDialog ) {


    this.subscribeAuthService();

  }

  ngOnInit(): void {
  }

  public toggleSideNavMenu(): void {

    this.clickSideNavToggle.emit();

  }

  public goToPage( pagePath: string ): void {

    this.router.navigate( ['/', pagePath] );

  }

  public openLoginRegisterDialog(): void {

    const dialogRef = this.dialog.open(LoginRegisterComponent);

  }

  // Auth User
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

  }


}
