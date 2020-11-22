import { Injectable } from '@angular/core';
import { CanActivate, ActivatedRouteSnapshot, RouterStateSnapshot, UrlTree } from '@angular/router';
import { Observable } from 'rxjs';
import { AuthService } from 'src/app/domain/services/auth.service';

@Injectable({
  providedIn: 'root'
})
export class PermissionGuard implements CanActivate {

  constructor(
    private authService: AuthService,
  ) {}

  canActivate( next: ActivatedRouteSnapshot ): Observable<boolean | UrlTree> | Promise<boolean | UrlTree> | boolean | UrlTree {

    const permission = next.data.permission;

    if (  this.authService.canUser(permission) ) {

      console.log('TEST Guard >>>> ', next.data.permission);
      return true;

    } else {

      return false;

    }


  }

}
