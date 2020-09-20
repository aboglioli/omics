import { Injectable } from '@angular/core';
import { CanActivate, ActivatedRouteSnapshot, UrlTree, Router } from '@angular/router';
import { Observable } from 'rxjs';
import { AuthService } from '../domain/services/auth.service';
import { NgxSpinnerService } from 'ngx-spinner';
import { PublicationService, IGetByIdResponse } from '../domain/services/publication.service';
import { map } from 'rxjs/operators';

@Injectable({
  providedIn: 'root'
})
export class PublicationOwnerGuard implements CanActivate {

  constructor(  private authService: AuthService,
                private spinnerService: NgxSpinnerService,
                private publicationService: PublicationService,
                private router: Router ) {}

  canActivate( next: ActivatedRouteSnapshot): Observable<boolean> | boolean {

    const publicationIdParam = next.paramMap.get('id');

    // Verificar primero si el usuario esta logueado
    if ( !this.authService.isLoggedIn() ) {
      return false;
    }


    // Saber si el usuario que accede es propietario de la publicación a verse
    this.spinnerService.show();
    return this.publicationService.getById(publicationIdParam).pipe(
      map((publicationRes: IGetByIdResponse) => {

        this.spinnerService.hide();

        const publicationAuthorId = publicationRes.publication.author_id;

        const isOwner = ( publicationAuthorId === this.authService.getIdUser() );

        if ( !isOwner ) {
          this.router.navigate(['/home']);
        }

        return isOwner;
      })
    );

  }

}
