import { Injectable } from '@angular/core';
import { CanActivate, ActivatedRouteSnapshot, UrlTree, Router } from '@angular/router';
import { Observable } from 'rxjs';
import { tap } from 'rxjs/operators';
import { AuthService } from 'src/app/domain/services/auth.service';
import { SweetAlertGenericMessageService } from '../services/sweet-alert-generic-message.service';

@Injectable({
  providedIn: 'root'
})
export class PermissionAnyGuard implements CanActivate {

  constructor(
    private authService: AuthService,
    private router: Router,
    private sweetAlertGenericService: SweetAlertGenericMessageService,
  ) {}

  canActivate( next: ActivatedRouteSnapshot ): Observable<boolean | UrlTree> | Promise<boolean | UrlTree> | boolean | UrlTree {

    const permission = next.data.permission;
    return this.authService.canUserAny(...permission).pipe(

      tap( res => {

        if ( res  ) {

          return true;

        } else {

          this.router.navigate(['/home']);
          this.sweetAlertGenericService.showAlertError('No tienes los permisos para ingresar', 'Error');
          return false;

        }

      } )

    );



  }

}
