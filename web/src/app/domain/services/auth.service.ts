import { HttpEvent, HttpHandler, HttpInterceptor, HttpRequest } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable, Subject, BehaviorSubject } from 'rxjs';
import { map, filter } from 'rxjs/operators';
import { Router } from '@angular/router';
import { IUser, can, canAny } from '../models/user';
import { IdentityService } from './identity.service';

@Injectable()
export class AuthService {

  private intialized = false;
  private authToken: string;

  public accessUser = new Subject<boolean>();
  public accessUser$ = this.accessUser.asObservable();

  public user = new BehaviorSubject<IUser>(null);
  public user$ = this.user.asObservable();

  constructor(
    private router: Router,
    private identityService: IdentityService,
  ) {

    const authToken = localStorage.getItem('auth_token');
    if (authToken) {
      this.authToken = authToken;
    }

    // const user = localStorage.getItem('user');
    // if (user) {
    //   this.user = new BehaviorSubject<IUser>(JSON.parse(user));
    // }

  }

  public getUser(): Observable<IUser> {
    if (!this.intialized) {
      this.loadUser();
      this.intialized = true;
    }

    return this.user
      .asObservable()
      .pipe(
        filter((user) => !!user && this.intialized)
      );
  }

  public loadUser(): void {
    this.identityService.getById('me', 'role').subscribe(
      (user: IUser) => {
        this.user.next(user);
        // localStorage.setItem('user', JSON.stringify(user));
      },
      (err: any) => {
        this.user.error(err);
        // localStorage.removeItem('user');
      },
    );
  }

  public canUser(...permissions: string[]): Observable<boolean> {
    return this.getUser().pipe(
      map((user: IUser) => can(user, ...permissions)),
    );
  }

  public canUserAny(...permissions: string[]): Observable<boolean> {
    return this.getUser().pipe(
      map((user: IUser) => canAny(user, ...permissions)),
    );
  }

  private updateStateSession( newValue: boolean ): void {

    this.accessUser.next( newValue );

  }

  public updateUserData( newValue: IUser ): void {

    this.user.next( newValue );

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
    // localStorage.removeItem('user');

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
