import { Injectable } from '@angular/core';
import { CanActivate, ActivatedRouteSnapshot, UrlTree, Router } from '@angular/router';
import { NgxSpinnerService } from 'ngx-spinner';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';
import { IUser } from '../domain/models/user';
import { IdentityService } from '../domain/services/identity.service';
import { SweetAlertGenericMessageService } from '../services/sweet-alert-generic-message.service';

@Injectable({
  providedIn: 'root'
})
export class ContentManagerGuard implements CanActivate {

  constructor(  private router: Router,
                private spinnerService: NgxSpinnerService,
                private identityService: IdentityService,
                private sweetAlertGenericService: SweetAlertGenericMessageService) {}

  canActivate( next: ActivatedRouteSnapshot ): Observable<boolean | UrlTree> | Promise<boolean | UrlTree> | boolean | UrlTree {

    this.spinnerService.show();
    return this.identityService.getById('me').pipe(

      map(( userRes: IUser ) => {

        const isAdmin = (userRes.role_id === 'admin' || userRes.role_id === 'content-manager');
        this.spinnerService.hide();

        if ( !isAdmin ) {
          this.sweetAlertGenericService.showAlertError('No tiene los permisos para accedera esta ruta', 'Error');
          this.router.navigate(['/home']);
        }

        return isAdmin;

      }

    ));


  }


}
