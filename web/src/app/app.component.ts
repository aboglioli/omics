import { Component, OnInit } from '@angular/core';
import { AuthService } from './domain/services/auth.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {

  public isAccessUserLogIn = false;  // Para habilitar algunas acciones según si esta el usuario logueado

  constructor(
    private authService: AuthService,
  ) {}

  ngOnInit(): void {

    this.isAccessUserLogIn = this.authService.isLoggedIn();
    this.subscribeAuthService();

  }

  private subscribeAuthService(): void {

    // Para comprobar en tiempo real si tiene o no acceso el usuario
    this.authService.accessUser$.subscribe( (data: boolean) => {

      // Para actualizar si el usuario no esta más logueado
      if ( data === false && this.isAccessUserLogIn ) {

        this.isAccessUserLogIn = false;

      } else {

        this.isAccessUserLogIn = data;

      }

    } );

  }

}
