import { Injectable } from '@angular/core';
import { CanActivate, ActivatedRouteSnapshot, RouterStateSnapshot, UrlTree, Router } from '@angular/router';
import { NgxSpinnerService } from 'ngx-spinner';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';
import { IUser } from '../domain/models/user';
import { IdentityService } from '../domain/services/identity.service';

@Injectable({
  providedIn: 'root'
})
export class AdminGuard implements CanActivate {

  constructor(  private router: Router,
                private spinnerService: NgxSpinnerService,
                private identityService: IdentityService) {}

  canActivate( next: ActivatedRouteSnapshot ): Observable<boolean | UrlTree> | Promise<boolean | UrlTree> | boolean | UrlTree {

    this.spinnerService.show();
    return this.identityService.getById('me').pipe(

      map(( userRes: IUser ) => {

        const isAdmin = (userRes.role_id === 'admin');
        this.spinnerService.hide();

        if ( !isAdmin ) {
          this.router.navigate(['/home']);
        }

        return isAdmin;

      }

    ));


  }


}
