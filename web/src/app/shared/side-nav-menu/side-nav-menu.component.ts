import { Component, OnInit, EventEmitter, Output, OnDestroy } from '@angular/core';
import { faChevronCircleLeft } from '@fortawesome/free-solid-svg-icons';
import { Router, NavigationStart, Event as NavigationEvent  } from '@angular/router';
import { Subscription } from 'rxjs';

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

  constructor(  private router: Router ) {}

  ngOnInit(): void {

    // Suscribirse para obtener los cambios en las rutas
    this.eventRoute$ = this.router.events.subscribe( (event: NavigationEvent)  => {

      if ( event instanceof NavigationStart ) {
        this.activePathSelected = event.url.substr(1);
        // console.log('TEST >', this.activePathSelected);
      }

    });

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

}
