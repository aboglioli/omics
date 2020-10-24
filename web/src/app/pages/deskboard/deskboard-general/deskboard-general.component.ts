import { Component, OnInit } from '@angular/core';
import { AuthService } from 'src/app/domain/services/auth.service';
import { IdentityService } from 'src/app/domain/services/identity.service';
import { IUser } from 'src/app/domain/models';
import { NgxSpinnerService } from 'ngx-spinner';
import {BreakpointObserver } from '@angular/cdk/layout';
import { Router } from '@angular/router';
import { faBookOpen, faChartPie, faWallet, faDesktop } from '@fortawesome/free-solid-svg-icons';
import { DeskboardOptionMenu, typeSearchCatalogue } from '../../../models/enums.model';


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
  public isBigScreen = true;

  public optionMenu = DeskboardOptionMenu;
  public currentOption = this.optionMenu.comics; // TODO: En vez de esto, debería usarse "patch child" (esto lo hice por tiempo)

  public optionTypeSearch = typeSearchCatalogue;
  public currentTypeSearch = this.optionTypeSearch.publication;

  constructor(
    private authService: AuthService,
    private identityService: IdentityService,
    private spinnerService: NgxSpinnerService,
    private breakpointObserver: BreakpointObserver,
    private router: Router,
  ) { }

  ngOnInit(): void {

    this.authService.authStart();

    this.checkWidthScreen();

    this.spinnerService.show();
    this.identityService.getById(this.authService.getIdUser() ).subscribe(
      (resData: IUser) => {

        this.userData = resData;
        this.spinnerService.hide();

      },
      (err: Error) => {

        console.error(err);
        this.spinnerService.hide();

      }
    )

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
