import { Component, OnInit, EventEmitter, Output, OnDestroy } from '@angular/core';
import { faChevronCircleLeft } from '@fortawesome/free-solid-svg-icons';
import { Router, NavigationStart, Event as NavigationEvent  } from '@angular/router';
import { Subscription } from 'rxjs';
import { AuthService } from 'src/app/domain/services/auth';

@Component({
  selector: 'app-side-nav-menu',
  templateUrl: './side-nav-menu.component.html',
  styleUrls: ['./side-nav-menu.component.scss']
})
export class SideNavMenuComponent implements OnInit, OnDestroy {

  @Output() clickSideNavToggle = new EventEmitter();
  @Output() clickRegisterLoginDialog = new EventEmitter();

  // Font Awseome icons
  public faBack = faChevronCircleLeft;

  // Suscriptos
  eventRoute$: Subscription;

  // Atributos propios
  activePathSelected: string;
  isAccessUserLogIn: boolean;  // Para habilitar algunas acciones según si esta el usuario logueado


  constructor(  private router: Router,
                private authService: AuthService ) {

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

  }

  ngOnDestroy(): void {

    this.eventRoute$.unsubscribe();

  }

  public closeSideNavMenu(): void {

    this.clickSideNavToggle.emit();

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

      }

    } );

  }

  public logout(): void {

    this.authService.logout();
    this.router.navigate(['/home']);

  }

}
