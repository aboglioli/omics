import { Injectable } from '@angular/core';
import { CanActivate, ActivatedRouteSnapshot, UrlTree, Router } from '@angular/router';
import { Observable } from 'rxjs';
import { AuthService } from '../domain/services/auth.service';

@Injectable({
  providedIn: 'root'
})
export class SameUserGuard implements CanActivate {

  constructor(  private authService: AuthService,
                private router: Router ) {}

  canActivate(next: ActivatedRouteSnapshot): Observable<boolean | UrlTree> | Promise<boolean | UrlTree> | boolean | UrlTree {

    const userIdParam = next.paramMap.get('id');

    if ( this.authService.getIdUser() === userIdParam ) {

      return true;

    } else {
      this.router.navigate(['/home']);
      return false;
    }

  }

}
