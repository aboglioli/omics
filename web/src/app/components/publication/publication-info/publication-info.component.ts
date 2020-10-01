import { Component, OnInit, Inject } from '@angular/core';
import { faTimesCircle, faBookmark, faMoneyBillAlt, faHeart, faEye, faStar } from '@fortawesome/free-solid-svg-icons';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { PublicationService } from '../../../domain/services/publication.service';
import { NgxSpinnerService } from 'ngx-spinner';
import { IGetByIdResponse } from '../../../domain/services/publication.service';
import { IPublication } from '../../../domain/models/publication';
import { IReaderPublicationInteraction } from '../../../domain/models/reader';
import { BreakpointObserver } from '@angular/cdk/layout';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';
import { Router } from '@angular/router';

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
  public publication: IPublication;
  public readerInfo: IReaderPublicationInteraction;

  public isBigScreen = true;
  public isReadButtonVisible: boolean;
  public totalLikes: number;

  constructor(
    public dialogRef: MatDialogRef<PublicationInfoComponent>,
    @Inject(MAT_DIALOG_DATA) public data: DialogData,
    private publicationService: PublicationService,
    private spinnerService: NgxSpinnerService,
    private breakpointObserver: BreakpointObserver,
    private sweetAlertGenericService: SweetAlertGenericMessageService,
    private router: Router,
  ) { }

  ngOnInit(): void {

    this.isReadButtonVisible =  (this.data.showRead) ? true : false;

    this.checkWidthScreen();

    // Cargar info de publicación
    this.spinnerService.show();
    this.publicationService.getById( this.data.idPublication,  'author, category').subscribe(
      (resPub: IGetByIdResponse ) => {

        console.log(resPub);
        this.publication = resPub.publication;
        this.readerInfo = resPub.reader;
        this.totalLikes = this.publication.statistics.likes;
        this.spinnerService.hide();

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
          console.log();
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

  public addReview(ratinSelected: number): void {

    console.log(ratinSelected);

  }

  public onGoToRead(): void {

    this.router.navigate( [`/read/${this.publication.id}`] );
    this.onClose();

  }

  public onGoToAuthorProfile(): void {
    this.router.navigate( [`/profile/${this.publication.author.id}`] );
    this.onClose();
  }

}
