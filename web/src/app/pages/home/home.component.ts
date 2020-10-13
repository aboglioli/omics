import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { MatDialog } from '@angular/material/dialog';
import { PasswordRewriteComponent } from '../../components/user/password-recovery/password-rewrite/password-rewrite.component';
import { ISearchResponse, PublicationService } from '../../domain/services/publication.service';
import { forkJoin } from 'rxjs';
import { IPublication } from '../../domain/models/publication';
import { NgxSpinnerService } from 'ngx-spinner';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';
import { PublicationInfoComponent } from 'src/app/components/publication/publication-info/publication-info.component';
import { BreakpointObserver } from '@angular/cdk/layout';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {

  private paramsToUse: any;
  public isBigScreen = true;

  public cardSize = '200px';

  // Lista de publicaciones
  public listTopPublcationNew: IPublication[];
  public listTopPublcationStars: IPublication[];
  public listTopPublcationLikes: IPublication[];
  public listTopPublcationViews: IPublication[];

  constructor(  private route: ActivatedRoute,
                private dialog: MatDialog,
                private spinnerService: NgxSpinnerService,
                private sweetAlertGenericService: SweetAlertGenericMessageService,
                private publicationService: PublicationService,
                private breakpointObserver: BreakpointObserver,
                private router: Router, ) { }

  ngOnInit(): void {

    this.checkWidthScreen();
    this.getAllTopPublication();

    if ( this.isRecoverPasswordNeeded() ) {

      this.showRecoverPasswrodModal();

    }

  }

  private getAllTopPublication(): void {

    this.spinnerService.show();

    const observableList =  [
      this.publicationService.search( { status: 'published', limit: 10, order_by: 'newest' }        , 'category' ),
      this.publicationService.search( { status: 'published', limit: 10, order_by: 'best_reviews' }  , 'category' ),
      this.publicationService.search( { status: 'published', limit: 10, order_by: 'most_liked' }    , 'category' ),
      this.publicationService.search( { status: 'published', limit: 10, order_by: 'most_viewed' }   , 'category' ),
    ];

    forkJoin( observableList).subscribe(

      ([ dataTopNew, dataTopStars, dataTopLikes, dataTopViews ]) => {

        this.spinnerService.hide();

        this.listTopPublcationNew = (dataTopNew as ISearchResponse).publications;
        this.listTopPublcationStars = (dataTopStars as ISearchResponse).publications;
        this.listTopPublcationLikes = (dataTopLikes as ISearchResponse).publications;
        this.listTopPublcationViews = (dataTopViews as ISearchResponse).publications;

        // console.log('Top New: ', this.listTopPublcationNew);
        // console.log('Top Stars', this.listTopPublcationStars);
        // console.log('Top Likes', this.listTopPublcationLikes);
        // console.log('Top Views', this.listTopPublcationViews);

      },
      (err: Error) => {

        this.spinnerService.hide();

        this.sweetAlertGenericService.showAlertError( 'Problemas con conexión al servidor al traer las publicaciones' );
        console.error(err);

      }
    );

  }

  public onGoToCatalogue(): void {

    this.router.navigate( ['/', 'catalogue'] );

  }


  public showPublicationInfo( idObra: string ): void {

    const dialogRef = this.dialog.open(
      PublicationInfoComponent,
      {
        panelClass: 'info-publication',
        data: {
          idPublication: idObra,
          showRead: true
        }
      }
    );

  }

  //#region Recover Password

  private isRecoverPasswordNeeded(): boolean {

    let isNeeded: boolean;

    this.route.params.subscribe( (params: any) => {

      isNeeded = (params.id) ? true : false;
      this.paramsToUse = params;

    });

    return isNeeded;

  }

  private showRecoverPasswrodModal(): void {

    const dialogRef = this.dialog.open(PasswordRewriteComponent, {
      data: {
              userId:  this.paramsToUse.id,
              temporalPass: this.paramsToUse.temporal_password,
              isRecoveryPassword: true
            },
      panelClass: 'no-padding-dialog'
    });

  }

  //#endregion

  private checkWidthScreen(): void {

    this.breakpointObserver.observe(['(max-width: 950px)']).subscribe(
      (result: any) => {

        if ( result.matches ) {
          this.isBigScreen = false;
          this.cardSize = '250px';
        } else {
          this.isBigScreen = true;
          this.cardSize = '200px';
        }

      });
  }

}
