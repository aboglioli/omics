import { Injectable } from '@angular/core';
import { CanActivate, ActivatedRouteSnapshot, RouterStateSnapshot, UrlTree, Router } from '@angular/router';
import { NgxSpinnerService } from 'ngx-spinner';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';
import { ICollection } from '../domain/models';
import { AuthService } from '../domain/services/auth.service';
import { CollectionService } from '../domain/services/collection.service';

@Injectable({
  providedIn: 'root'
})
export class CollectionOwnerGuard implements CanActivate {
  constructor(  private authService: AuthService,
                private spinnerService: NgxSpinnerService,
                private collectionService: CollectionService,
                private router: Router ) {}

  canActivate( next: ActivatedRouteSnapshot): Observable<boolean> | boolean {

    const collectionIdParam = next.paramMap.get('id');

    // Verificar primero si el usuario esta logueado
    if ( !this.authService.isLoggedIn() ) {
      return false;
    }


    // Saber si el usuario que accede es propietario de la colección a verse
    this.spinnerService.show();
    return this.collectionService.getById(collectionIdParam).pipe(
    map((collectionRes: ICollection) => {

        this.spinnerService.hide();

        const collectionAuthorId = collectionRes.author_id;
        const isOwner = ( collectionAuthorId === this.authService.getIdUser() );

        if ( !isOwner ) {
          this.router.navigate(['/home']);
        }

        return isOwner;
      })
    );

    }
}
