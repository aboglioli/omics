import { Component, OnInit, EventEmitter, Output, OnDestroy } from '@angular/core';
import { faChevronCircleLeft } from '@fortawesome/free-solid-svg-icons';
import { Router, NavigationStart, Event as NavigationEvent  } from '@angular/router';
import { Subscription } from 'rxjs';
import { AuthService } from '../../domain/services/auth.service';
import { IUser, can, canAny } from 'src/app/domain/models';
import { IdentityService } from 'src/app/domain/services/identity.service';

@Component({
  selector: 'app-side-nav-menu-main',
  templateUrl: './side-nav-menu-main.component.html',
  styleUrls: ['./side-nav-menu-main.component.scss']
})
export class SideNavMenuMainComponent implements OnInit, OnDestroy {

  @Output() clickSideNavMainToggle = new EventEmitter();
  @Output() clickRegisterLoginDialog = new EventEmitter();

  // Font Awseome icons
  public faBack = faChevronCircleLeft;

  // Suscriptos
  eventRoute$: Subscription;

  // Atributos propios
  activePathSelected: string;
  isAccessUserLogIn: boolean;  // Para habilitar algunas acciones según si esta el usuario logueado

  public userData: IUser;
  private userId: string;

  public can = can;
  public canAny = canAny;

  constructor(  private router: Router,
                private authService: AuthService,
                private identifyService: IdentityService ) {

    this.subscribeAuthService();

  }

  ngOnInit(): void {

    // Suscribirse para obtener los cambios en las rutas
    this.eventRoute$ = this.router.events.subscribe( (event: NavigationEvent)  => {

      if ( event instanceof NavigationStart ) {
        this.activePathSelected = event.url.substr(1);
      }

    });

    this.isAccessUserLogIn = this.authService.isLoggedIn();

    if ( this.isAccessUserLogIn  ) {

      this.userId = this.authService.getIdUser();
      this.getUserDataFromService(this.userId);

    }

  }

  ngOnDestroy(): void {

    this.eventRoute$.unsubscribe();

  }

  public closeSideNavMenu(): void {

    this.clickSideNavMainToggle.emit();

  }

  public goToPage( pagePath: string ): void {

    this.router.navigate( ['/', pagePath] );
    this.activePathSelected = pagePath;

    this.closeSideNavMenu();

  }

  public openRegisterLoginDialog(): void {

    this.closeSideNavMenu();
    this.clickRegisterLoginDialog.emit();

  }

  private getUserDataFromService( id: string ): void {

    this.authService.getUser().subscribe((user) => {
      this.userData = user;
    });

  }

  // Auth User
  private subscribeAuthService(): void {

    // Para comprobar en tiempo real si tiene o no acceso el usuario
    this.authService.accessUser$.subscribe( (data: boolean) => {

      // Para actualizar si el usuario no esta más logueado
      if ( data === false && this.isAccessUserLogIn ) {

        this.isAccessUserLogIn = false;
        this.logout();

      } else {

        this.isAccessUserLogIn = data;
        this.userId = this.authService.getIdUser();
        this.getUserDataFromService(this.userId);

      }

    } );

  }

  public logout(): void {

    this.authService.logout();
    this.router.navigate(['/home']);

  }

}
