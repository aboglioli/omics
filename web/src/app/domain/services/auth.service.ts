import { HttpEvent, HttpHandler, HttpInterceptor, HttpRequest } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable, Subject } from 'rxjs';
import { Router } from '@angular/router';

@Injectable()
export class AuthService {

  private authToken: string;

  public accessUser = new Subject<boolean>();
  public accessUser$ = this.accessUser.asObservable();

  constructor(
    private router: Router,
  ) {

    const authToken = localStorage.getItem('auth_token');
    if (authToken) {
      this.authToken = authToken;
    }

  }

  private updateStateSession( newValue: boolean ): void {

    this.accessUser.next( newValue );

  }

  public setToken(authToken: string, idUser: string): void {

    this.authToken = authToken;
    localStorage.setItem('auth_token', this.authToken);
    localStorage.setItem('id_user', idUser);

    this.updateStateSession( true );

  }

  public getIdUser(): string {

    return localStorage.getItem('id_user');

  }

  public logout(): void {

    this.authToken = null;
    this.updateStateSession(false);
    this.authToken = null;
    localStorage.removeItem('auth_token');
    localStorage.removeItem('id_user');

  }

  public getToken(): string {
    return this.authToken;
  }

  public isLoggedIn(): boolean {
    return !!this.authToken;
  }

  public authStart(): void {

    // Saber si el usuario esta o no logueado para rediriguir
    if ( !this.isLoggedIn() ) {

      this.router.navigate(['/home']);
      this.updateStateSession(false);

    } else {

      this.updateStateSession(true);

    }

  }

}

@Injectable()
export class AuthInterceptor implements HttpInterceptor {
  constructor(private authServ: AuthService) {  }

  public intercept(req: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
    if (this.authServ.isLoggedIn()) {
      req = req.clone({
        setHeaders: {
          Accept: 'application/json',
          Authorization: `Bearer ${this.authServ.getToken()}`,
          // 'Content-Type': 'application/json; charset=utf-8',
        },
      });
    }

    return next.handle(req);
  }
}
