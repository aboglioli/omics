import { Component, OnInit } from '@angular/core';
import { NgxSpinnerService } from 'ngx-spinner';
import { PublicationService } from 'src/app/domain/services/publication.service';
import { IPublication, IPage } from '../../domain/models/publication';
import { IGetByIdResponse, IReadResponse } from '../../domain/services/publication.service';
import { faChevronLeft, faChevronRight, faMoneyBillAlt, faBookmark, faInfoCircle, faHeart, faCommentDots  } from '@fortawesome/free-solid-svg-icons';
import { ActivatedRoute } from '@angular/router';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';
import { IReaderPublicationInteraction } from '../../domain/models/reader';

@Component({
  selector: 'app-visor-comic',
  templateUrl: './visor-comic.component.html',
  styleUrls: ['./visor-comic.component.scss']
})
export class VisorComicComponent implements OnInit {

  // Font Awseome icons
  public faLeft = faChevronLeft;
  public faRight = faChevronRight;
  public faDonar = faMoneyBillAlt;
  public faLike = faHeart;
  public faFavorito = faBookmark;
  public faInfo = faInfoCircle;
  public faComentario = faCommentDots;

  // Manejo de publicación
  public publicationToShow: IPublication;
  private publicationId: string;

  public readerInfo: IReaderPublicationInteraction;

  // Manejo de pagina
  public pagesList: IPage[];
  public pagesTotal: number;

  public pageCurrent: number;
  public pageToShow: string;

  constructor(
    private spinnerService: NgxSpinnerService,
    private publicationService: PublicationService,
    private activateRoute: ActivatedRoute,
    private sweetAlertGenericService: SweetAlertGenericMessageService,
  ) { }

  ngOnInit(): void {

    this.pagesTotal = 0;
    this.pageCurrent = 0;

    this.getPublicationDataByParams();
  }

  private getPublicationDataByParams(): void {

    this.spinnerService.show();
    this.activateRoute.params.subscribe( params => {

      this.publicationService.getById(params.id).subscribe(

        (resPub: IGetByIdResponse) => {

          this.publicationToShow = resPub.publication;
          this.readerInfo = resPub.reader;
          console.log(resPub);

          this.publicationId = params.id;
          this.publicationService.read( this.publicationId ).subscribe(

            (resPages: IReadResponse) => {

              this.pagesList = resPages.pages;
              this.pagesTotal = this.pagesList.length;

              this.setCurrentPage(this.pageCurrent);

              this.spinnerService.hide();

            },
            (err: Error) => {
              console.error(err);
            }
          );


        },
        (err: Error ) => {
          console.error(err);
          this.spinnerService.hide();
        }

      );
    });

  }

  public setCurrentPage( numberPage: number ): void {

    if (  numberPage >= 0 && numberPage < this.pagesTotal ) {
      this.pageToShow = this.pagesList[numberPage].images[0].url;
      this.pageCurrent = numberPage;
    }

  }

  public onDonar(): void {
    this.sweetAlertGenericService.showUnderConstrucction();
  }

  public onFavorito(): void {
    this.sweetAlertGenericService.showUnderConstrucction();
  }

  public onLike(): void {
    this.sweetAlertGenericService.showUnderConstrucction();
  }

  public onComentarios(): void {
    this.sweetAlertGenericService.showUnderConstrucction();
  }

  public onInfo(): void {

    this.publicationService.like( this.publicationId );

  }

}
