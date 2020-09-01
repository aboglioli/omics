import { Injectable } from '@angular/core';
import { CanActivate, ActivatedRouteSnapshot, RouterStateSnapshot, UrlTree, Router } from '@angular/router';
import { Observable } from 'rxjs';
import { AuthService } from '../domain/services/auth';
@Injectable({
  providedIn: 'root'
})
export class AuthNotLoginGuard implements CanActivate {

  constructor(  private authService: AuthService,
                private router: Router ) {}

  // En caso de no estar logueado, redirigir al home
  canActivate(
    next: ActivatedRouteSnapshot,
    state: RouterStateSnapshot): Observable<boolean | UrlTree> | Promise<boolean | UrlTree> | boolean | UrlTree {

      if (  this.authService.isLoggedIn() ) {

        this.router.navigate(['/home']);
        return false;

      } else {

        return true;

      }


  }

}
