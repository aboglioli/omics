import { Component, OnInit, Output, EventEmitter,  } from '@angular/core';
import { faChevronCircleRight } from '@fortawesome/free-solid-svg-icons';
import { Router, NavigationStart, Event as NavigationEvent  } from '@angular/router';
import { AuthService } from 'src/app/domain/services/auth.service';
import { IdentityService } from '../../domain/services/identity.service';
import { SubscriptionService } from '../../domain/services/subscription.service';
import { ReaderService } from '../../domain/services/reader.service';
import { IUser } from '../../domain/models/user';

@Component({
  selector: 'app-side-nav-menu-user',
  templateUrl: './side-nav-menu-user.component.html',
  styleUrls: ['./side-nav-menu-user.component.scss']
})
export class SideNavMenuUserComponent implements OnInit {

  @Output() clickSideNavUserToggle = new EventEmitter();
  @Output() clickLogout = new EventEmitter();

  // Font Awseome icons
  public faBack = faChevronCircleRight;

  // Atributos propios
  public activePathSelected: string;
  public userData: IUser;
  private userId: string;
  public readerIsSubscribed = false;

  constructor(
    private router: Router,
    private authService: AuthService,
    private identifyService: IdentityService,
    private readerService: ReaderService,
    private subscriptionService: SubscriptionService,
  ) {

  }

  ngOnInit(): void {

    this.userId = this.authService.getIdUser();
    this.setSubscribe();

    this.readerService.getSubscription(this.userId).subscribe(
      (res) => {
        if (res && res.status.status === 'active') {
          this.readerIsSubscribed = true;
        }
      },
      (err) => {
        console.log(err);
      },
    );

  }

  private setSubscribe(): void {

    this.subscribeAuthService();
    this.getUserDataFromService( this.userId );

    this.router.events.subscribe( (event: NavigationEvent) => {

      if (event instanceof NavigationStart) {

        this.setCurrentPathProfile( event.url );

      }

    });


  }

  private getUserDataFromService( id: string ): void {

    this.identifyService.getById(id).subscribe( (data: IUser) => {

      this.userData = data;

    } );

  }

  private setCurrentPathProfile( url: string ): void {

    // TODO: Sé que debe haber una forma mejor, pero es la única que logre comprender y aplicar

    let profileString = url.split('/')[1];

    // En caso que no sea nada de profile
    if ( profileString !== 'profile' ){
      this.activePathSelected = '';
    } else {

      // Detectar si está viendo su perfil o editandolo
      profileString = url.split('/')[3];

      if ( profileString ===  'editUser') {

        this.activePathSelected = 'profileEdit';

      } else {

        profileString = url.split('/')[2]; // Saber si el usuario que ve es el suyo

        if ( profileString === this.userId  ) {

          this.activePathSelected = 'userProfile';

        }

      }

    }


  }

  public goToPlanPage(): void {
    this.router.navigate(['/plans']);
    this.closeSideNavMenu();
  }

  public goToUserPage( isEdit: boolean ): void {

    let goToUrl = `/profile/${this.userId}`;

    if ( isEdit ) {
      goToUrl += '/editUser';
    }

    this.router.navigate( [ goToUrl ]);
    this.closeSideNavMenu();

  }

  public closeSideNavMenu(): void {

    this.clickSideNavUserToggle.emit();

  }


  // Auth User
  private subscribeAuthService(): void {

    // Para tener en cuenta cambios al loguearse
    this.authService.accessUser$.subscribe( (data: boolean) => {

      // Para actualizar si el usuario no esta más logueado
      if ( data === true  ) {

        this.userId = this.authService.getIdUser();
        this.getUserDataFromService( this.userId );

      }

    } );

  }

  public unsubscribe(): void {
    this.subscriptionService.unsubscribe().subscribe(
      (res) => {
        this.readerIsSubscribed = false;
        // console.log(res);
      },
      (err) => {
        console.error(err);
      }
    )
  }

  public logout(): void {
    this.clickLogout.emit();
    this.closeSideNavMenu();
  }

}
