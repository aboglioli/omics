import { Component, Input, OnInit, OnChanges, SimpleChanges } from '@angular/core';
import { faFileUpload, faPlusCircle } from '@fortawesome/free-solid-svg-icons';
import { Router, ActivatedRoute } from '@angular/router';
import { AuthorService } from '../../../domain/services/author.service';
import { NgxSpinnerService } from 'ngx-spinner';
import { IGetPublicationsResponse } from 'src/app/domain/services/collection.service';
import { IPublication } from '../../../domain/models/publication';
import { typeSearchCatalogue } from 'src/app/models/enums.model';
import { ICollection } from 'src/app/domain/models';

@Component({
  selector: 'app-deskboard-mis-comics',
  templateUrl: './deskboard-mis-comics.component.html',
  styleUrls: ['./deskboard-mis-comics.component.scss']
})
export class DeskboardMisComicsComponent implements OnInit, OnChanges {

  @Input() typeSearch: typeSearchCatalogue = typeSearchCatalogue.publication;

  // Font Awseome icons
  public faUpload = faFileUpload;
  public faAdd = faPlusCircle;

  public publicationList: IPublication[];
  public collectionList: ICollection[];

  public typeSearchList = typeSearchCatalogue;

  constructor(
    private router: Router,
    private activatedRoute: ActivatedRoute,
    private authorService: AuthorService,
    private spinnerService: NgxSpinnerService,
  ) { }

  ngOnInit(): void {

  }

  ngOnChanges( changes: SimpleChanges ): void {

    if ( changes.typeSearch ) {

      ( this.typeSearch === typeSearchCatalogue.publication  ) ?
        this.getPublicationData() :
        this.getCollectionData();

    }

  }

  public goToNewPublication(): void {

    this.router.navigate(['publication/new'], { relativeTo: this.activatedRoute });

  }

  public goToNewCollection(): void {

    this.router.navigate(['collection/new'], { relativeTo: this.activatedRoute });

  }

  private getPublicationData(): void {

    this.spinnerService.show();

    this.authorService.getPublications('me').subscribe(
      (resData) => {

        this.publicationList = resData.items;

        this.spinnerService.hide();

      },
      (err: Error) => {

        console.error(err);
        this.spinnerService.hide();

      }
    );

  }

  private getCollectionData(): void {

    this.spinnerService.show();

    this.authorService.getCollections('me').subscribe(
      (resData) => {

        this.collectionList = resData.items;
        this.spinnerService.hide();

      },
      (err: Error) => {

        console.error(err);
        this.spinnerService.hide();

      }
    );

  }

}
