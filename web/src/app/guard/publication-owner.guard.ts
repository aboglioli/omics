import { Injectable } from '@angular/core';
import { CanActivate, ActivatedRouteSnapshot, UrlTree, Router } from '@angular/router';
import { Observable } from 'rxjs';
import { AuthService } from '../domain/services/auth.service';
import { NgxSpinnerService } from 'ngx-spinner';
import { PublicationService, IGetByIdResponse } from '../domain/services/publication.service';
import { map } from 'rxjs/operators';
import { SweetAlertGenericMessageService } from '../services/sweet-alert-generic-message.service';

@Injectable({
  providedIn: 'root'
})
export class PublicationOwnerGuard implements CanActivate {

  constructor(  private authService: AuthService,
                private spinnerService: NgxSpinnerService,
                private publicationService: PublicationService,
                private sweetAlertGenericService: SweetAlertGenericMessageService,
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
          this.sweetAlertGenericService.showAlertError('No es dueño de la publicación', 'Error');
          this.router.navigate(['/home']);
        }

        return isOwner;
      })
    );

  }

}
