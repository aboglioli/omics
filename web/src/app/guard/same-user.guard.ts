import { Injectable } from '@angular/core';
import { CanActivate, ActivatedRouteSnapshot, UrlTree, Router } from '@angular/router';
import { Observable } from 'rxjs';
import { AuthService } from '../domain/services/auth.service';
import { SweetAlertGenericMessageService } from '../services/sweet-alert-generic-message.service';

@Injectable({
  providedIn: 'root'
})
export class SameUserGuard implements CanActivate {

  constructor(  private authService: AuthService,
                private sweetAlertGenericService: SweetAlertGenericMessageService,
                private router: Router ) {}

  canActivate(next: ActivatedRouteSnapshot): Observable<boolean | UrlTree> | Promise<boolean | UrlTree> | boolean | UrlTree {

    const userIdParam = next.paramMap.get('id');

    if ( this.authService.getIdUser() === userIdParam ) {

      return true;

    } else {
      this.router.navigate(['/home']);
      this.sweetAlertGenericService.showAlertError('No es dueño del perfil que desea acceder', 'Error');
      return false;
    }

  }

}
