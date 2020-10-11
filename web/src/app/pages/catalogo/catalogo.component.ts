import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { PublicationService, ISearchResponse as PublicationSearchResponnse  } from '../../domain/services/publication.service';
import { IPublication } from '../../domain/models/publication';
import { NgxSpinnerService } from 'ngx-spinner';
import { PublicationInfoComponent } from '../../components/publication/publication-info/publication-info.component';
import { MatDialog } from '@angular/material/dialog';
import { typeSearchCatalogue } from 'src/app/models/enums.model';
import { CollectionService, ISearchResponse } from '../../domain/services/collection.service';
import { ICollection } from '../../domain/models/collection';

@Component({
  selector: 'app-catalogo',
  templateUrl: './catalogo.component.html',
  styleUrls: ['./catalogo.component.scss']
})
export class CatalogoComponent implements OnInit {

  publicationList: IPublication[] = [];
  collectionList: ICollection[] = [];

  public optionTypeSearch = typeSearchCatalogue;
  public currentTypeSearch = this.optionTypeSearch.publication;

  // TODO: Separar el catalogo de los filtros para que se comuniquen entre sí  (para simplificar código)

  constructor(
    private router: Router,
    private publicationService: PublicationService,
    private collectionService: CollectionService,
    private spinnerService: NgxSpinnerService,
    private dialog: MatDialog,
  ) { }

  ngOnInit(): void {

    this.spinnerService.show();
    this.changeTypeSearch( this.currentTypeSearch );

  }

  public changeTypeSearch(type: typeSearchCatalogue): void {

    this.currentTypeSearch = type;
    ( this.currentTypeSearch === typeSearchCatalogue.publication  ) ?
      this.getPublicationData() :
      this.getCollectionData();

  }

  private getPublicationData(): void {

    this.publicationService.search( { status: 'published' }, 'category' ).subscribe(
      (searchRes: PublicationSearchResponnse ) => {

        this.publicationList = searchRes.publications;
        this.spinnerService.hide();


      },
      (err: Error) =>  {

        console.error(err);
        this.spinnerService.hide();

      }
    );

  }

  private getCollectionData(): void {


    this.collectionService.search( {} ).subscribe(
      (searchRes: ISearchResponse ) => {

        this.collectionList = searchRes.collections;
        this.spinnerService.hide();

      },
      (err: Error) =>  {

        console.error(err);
        this.spinnerService.hide();

      }
    );

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

  public goToObra( idObra: string ): void {

    this.router.navigate( [`/read/${idObra}`] );

  }

}
