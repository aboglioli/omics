import { Component, OnInit, Inject } from '@angular/core';
import { faTimesCircle, faBookmark, faMoneyBillAlt, faHeart, faEye, faStar } from '@fortawesome/free-solid-svg-icons';
import { MatDialogRef, MAT_DIALOG_DATA, MatDialog } from '@angular/material/dialog';
import { IGetReviewsResponse, PublicationService } from '../../../domain/services/publication.service';
import { NgxSpinnerService } from 'ngx-spinner';
import { IGetByIdResponse } from '../../../domain/services/publication.service';
import { IPublication } from '../../../domain/models/publication';
import { IReaderPublicationInteraction } from '../../../domain/models/reader';
import { BreakpointObserver } from '@angular/cdk/layout';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';
import { Router } from '@angular/router';
import { PublicationReviewAddComponent } from '../publication-review-add/publication-review-add.component';
import { IReview } from '../../../domain/models/review';
import { ReaderService } from '../../../domain/services/reader.service';
import { AuthService } from '../../../domain/services/auth.service';
import { IContract } from '../../../domain/models/contract';

export interface DialogData {
  idPublication: string;
  showRead: boolean;
}

@Component({
  selector: 'app-comic-info',
  templateUrl: './publication-info.component.html',
  styleUrls: ['./publication-info.component.scss']
})

export class PublicationInfoComponent implements OnInit {

  // Font Awseome icons
  public faClose = faTimesCircle;
  public faDonar = faMoneyBillAlt;
  public faLike = faHeart;
  public faFavorito = faBookmark;
  public faVistas = faEye;
  public faStarFill = faStar;

  public ratingPublication = 0;
  private oldRatingPublication = this.ratingPublication;
  public publication: IPublication;
  public contract: IContract;
  public readerInfo: IReaderPublicationInteraction;

  public reviewList: IReview[];

  public isBigScreen = true;
  public isReadButtonVisible: boolean;
  public totalLikes: number;

  public readerIsSubscribed = false;
  public canRequestContract = false;
  public readerIsAuthor = false;

  constructor(
    public dialogRef: MatDialogRef<PublicationInfoComponent>,
    @Inject(MAT_DIALOG_DATA) public data: DialogData,
    private publicationService: PublicationService,
    private readerService: ReaderService,
    private authService: AuthService,
    private spinnerService: NgxSpinnerService,
    private breakpointObserver: BreakpointObserver,
    private sweetAlertGenericService: SweetAlertGenericMessageService,
    private router: Router,
    private dialog: MatDialog,
  ) { }

  ngOnInit(): void {

    this.isReadButtonVisible =  (this.data.showRead) ? true : false;

    this.checkWidthScreen();

    // Cargar info de publicación
    this.spinnerService.show();

    this.getPublicationInfo();

    this.readerService.getSubscription('me').subscribe(
      (res) => {
        this.readerIsSubscribed = res.status.status === 'active';
      },
      (err) => {
        console.log(err);
      },
    )

  }

  private getPublicationInfo(): void {

    this.publicationService.getById( this.data.idPublication,  'author, category').subscribe(
      (resPub: IGetByIdResponse ) => {

        const loggedInUserId = this.authService.getIdUser();
        this.readerIsAuthor = resPub.publication.author.id === loggedInUserId;

        this.publicationService.canRequestContract(this.data.idPublication).subscribe(
          (res) => {
            this.canRequestContract = res.can_request;
          },
        );

        this.publicationService.getContract(this.data.idPublication).subscribe(
          (res) => {
            this.contract = res;
          },
        )

        // console.log('TEST > ', resPub);

        //#region Obtener info general de la publicación
        this.publication = resPub.publication;
        this.readerInfo = resPub.reader;
        this.totalLikes = this.publication.statistics.likes;

        if ( this.readerInfo  ) {
          this.ratingPublication = (this.readerInfo.review) ? this.readerInfo.review.stars : 0;
        }

        this.oldRatingPublication = this.ratingPublication;

        //#endregion

        //#region Obtener información de reviews de esta publicación
        this.publicationService.getReviews( this.data.idPublication ).subscribe(
          ( resReviews: IGetReviewsResponse ) => {

            this.reviewList = resReviews.reviews;
            this.spinnerService.hide();

            // console.log('TEST > ', this.reviewList);

          },
          (err: Error) => {

            console.error(err);
            this.spinnerService.hide();

          }
        )
        //#endregion


      },
      ( err: Error) => {

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

  public onClose(): void {
    this.dialogRef.close();
  }

  public onDonar(): void {
    this.sweetAlertGenericService.showUnderConstrucction();
  }

  public onFavorito(): void {

    if ( !this.readerInfo.in_favorites  ) {

      this.publicationService.addToFavorites( this.publication.id ).subscribe(
        (res: any) =>  {

          this.readerInfo.in_favorites = true;
        }
      );

    } else {

      this.publicationService.removeFromFavorites( this.publication.id ).subscribe(
        (res: any) => {
          this.readerInfo.in_favorites = false;
        }
      );

    }

  }

  public onLike(): void {

    if (   this.readerInfo.liked  ) {

      this.publicationService.unlike( this.publication.id ).subscribe(
        (res: any) =>  {

          this.readerInfo.liked = false;
          this.totalLikes--;
        }
      );

    } else {

      this.publicationService.like( this.publication.id ).subscribe(
        (res: any) => {
          this.readerInfo.liked = true;
          this.totalLikes++;
        }
      );

    }

  }

  public addReview(ratingSelected: number): void {

    const dialogRef = this.dialog.open(
      PublicationReviewAddComponent,
      {
        panelClass: 'no-padding-dialog',
        data: {
          rating: ratingSelected,
          idPublication: this.data.idPublication,
          review: (this.readerInfo) ? this.readerInfo.review : null
        }
      }
    );

    dialogRef.afterClosed().subscribe( resReviewChanged => {

      if ( resReviewChanged ) {
        this.getPublicationInfo();
      } else {
        this.ratingPublication = this.oldRatingPublication;
      }

    });

  }

  public onGoToRead(): void {

    this.router.navigate( [`/read/${this.publication.id}`] );
    this.onClose();

  }

  public onGoToAuthorProfile(): void {
    this.router.navigate( [`/profile/${this.publication.author.id}`] );
    this.onClose();
  }

  public requestContract(): void {
    this.publicationService.requestContract(this.publication.id).subscribe(
      (res) => {
        this.getPublicationInfo();
      }
    )
  }

}
