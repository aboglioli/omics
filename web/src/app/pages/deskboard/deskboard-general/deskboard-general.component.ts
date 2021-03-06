import { Component, OnInit } from '@angular/core';
import { AuthService } from 'src/app/domain/services/auth.service';
import { IdentityService } from 'src/app/domain/services/identity.service';
import { IUser, can, Flags } from 'src/app/domain/models';
import { NgxSpinnerService } from 'ngx-spinner';
import {BreakpointObserver } from '@angular/cdk/layout';
import { Router } from '@angular/router';
import { faBookOpen, faChartPie, faWallet, faDesktop } from '@fortawesome/free-solid-svg-icons';
import { DeskboardOptionMenu, typeSearchCatalogue } from '../../../models/enums.model';
import { AuthorService } from '../../../domain/services/author.service';
import { SweetAlertGenericMessageService } from '../../../services/sweet-alert-generic-message.service';


@Component({
  selector: 'app-deskboard-general',
  templateUrl: './deskboard-general.component.html',
  styleUrls: ['./deskboard-general.component.scss']
})
export class DeskboardGeneralComponent implements OnInit {

  // FontAwesome Icon
  public faComic = faBookOpen;
  public faReporte = faChartPie;
  public faBilletera = faWallet;
  public faDesk = faDesktop;

  public userData: IUser;
  public can = can;
  public isBigScreen = true;

  public optionMenu = DeskboardOptionMenu;
  public currentOption = this.optionMenu.comics; // TODO: En vez de esto, debería usarse "patch child" (esto lo hice por tiempo)

  public optionTypeSearch = typeSearchCatalogue;
  public currentTypeSearch = this.optionTypeSearch.publication;

  constructor(
    private authService: AuthService,
    private identityService: IdentityService,
    private authorService: AuthorService,
    private spinnerService: NgxSpinnerService,
    private breakpointObserver: BreakpointObserver,
    private sweetAlertService: SweetAlertGenericMessageService,
    private router: Router,
  ) { }

  ngOnInit(): void {

    this.authService.authStart();

    this.checkWidthScreen();

    this.spinnerService.show();
    this.identityService.getById(this.authService.getIdUser(), 'role' ).subscribe(
      (resData: IUser) => {

        this.userData = resData;

        if (this.userData.flag === Flags.New) {
          this.authorService.getById('me').subscribe(({ author }) => {
            if (author.publications > 0) {
              this.sweetAlertService.showAlertSuccess(
                'Tu primera publicación fue aprobada. Gracias por formar parte de Omics.',
                'Felicitaciones',
              );

              this.identityService.setFlag('me', { flag: Flags.Welcomed }).subscribe();
            }
          });
        }


        this.spinnerService.hide();
      },
      (err: Error) => {

        console.error(err);
        this.spinnerService.hide();

      }
    );

  }

  private checkWidthScreen(): void {

    this.breakpointObserver.observe(['(max-width: 950px)']).subscribe(
      (result: any) => {

        this.isBigScreen = (result.matches) ? false : true;

      });
  }

  public onGoToProfile(): void {

    this.router.navigate( [`/profile/${this.userData.id}`]);

  }

  public onChangeTopMenu( option: number ): void {

    this.currentOption = option;

  }

  public onMyPublicationTypeSearch( option: number ): void {
    this.currentTypeSearch = option;
  }


}
