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
  access: boolean;  // Para habilitar algunas acciones según si esta el usuario logueado


  constructor(  private router: Router,
                private authService: AuthService,
                public dialog: MatDialog ) {



    // Para comprobar en tiempo real si tiene o no acceso el usuario
    this.authService.accessUser$.subscribe( (data: boolean) => {

      console.log('session state: ', data);

      // Para actualizar si el usuario no esta más logueado
      if ( data === false && this.access ) {

        this.access = false;
        this.logout();

      } else {

        this.access = data;

      }

    } );


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

  public logout(): void {

    this.authService.logout();

  }


}
