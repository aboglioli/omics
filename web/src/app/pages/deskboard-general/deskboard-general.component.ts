import { Component, OnInit } from '@angular/core';
import { AuthService } from 'src/app/domain/services/auth.service';
import { IdentityService } from 'src/app/domain/services/identity.service';
import { IUser } from 'src/app/domain/models';
import { NgxSpinnerService } from 'ngx-spinner';
import {BreakpointObserver } from '@angular/cdk/layout';
import { Router } from '@angular/router';
import { faBookOpen, faChartPie, faCommentDots, faWallet, faDesktop } from '@fortawesome/free-solid-svg-icons';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';

@Component({
  selector: 'app-deskboard-general',
  templateUrl: './deskboard-general.component.html',
  styleUrls: ['./deskboard-general.component.scss']
})
export class DeskboardGeneralComponent implements OnInit {

  // FontAwesome Icon
  public faComic = faBookOpen;
  public faReporte = faChartPie;
  public faComentarios = faCommentDots;
  public faBilletera = faWallet;
  public faDesk = faDesktop;

  public userData: IUser;
  public isBigScreen = true;
  public idSubComponent = 0; // TODO: En vez de esto, debería usarse "patch child" (esto lo hice por tiempo)

  constructor(
    private authService: AuthService,
    private identityService: IdentityService,
    private spinnerService: NgxSpinnerService,
    private breakpointObserver: BreakpointObserver,
    private router: Router,
    private sweetAlertGenericService: SweetAlertGenericMessageService,
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

    this.breakpointObserver.observe(['(max-width: 900px)']).subscribe(
      (result: any) => {

        this.isBigScreen = (result.matches) ? false : true;

      });
  }

  public onGoToProfile(): void {

    this.router.navigate( [`/profile/${this.userData.id}`]);

  }

  public onMyComics(): void {
    this.idSubComponent = 0;
    this.sweetAlertGenericService.showUnderConstrucction();
  }

  public onReports(): void {
    // this.idSubComponent = 1;
    this.sweetAlertGenericService.showUnderConstrucction();
  }

  public onComments(): void {
    // this.idSubComponent = 2;
    this.sweetAlertGenericService.showUnderConstrucction();
  }

  public onWallet(): void {
    // this.idSubComponent = 3;
    this.sweetAlertGenericService.showUnderConstrucction();
  }

}
