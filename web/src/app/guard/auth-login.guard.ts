import { Injectable } from '@angular/core';
import { CanActivate, UrlTree, Router } from '@angular/router';
import { Observable } from 'rxjs';
import { AuthService } from '../domain/services/auth.service';
import { SweetAlertGenericMessageService } from '../services/sweet-alert-generic-message.service';

@Injectable({
  providedIn: 'root'
})
export class AuthLoginGuard implements CanActivate {

  constructor(  private authService: AuthService,
                private router: Router,
                private sweetAlertGenericService: SweetAlertGenericMessageService ) {}

  canActivate(): Observable<boolean | UrlTree> | Promise<boolean | UrlTree> | boolean | UrlTree {

      if (  this.authService.isLoggedIn() ) {

        return true;

      } else {

        this.sweetAlertGenericService.showAlertError('Necesita ingresar al sistema para acceder a este sitio', 'Error');
        this.router.navigate(['/home']);
        return false;

      }


  }

}
