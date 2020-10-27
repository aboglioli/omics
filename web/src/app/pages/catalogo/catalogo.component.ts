import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { PublicationService, ISearchCommand as ISearchCommandPublication } from '../../domain/services/publication.service';
import { IPublication } from '../../domain/models/publication';
import { NgxSpinnerService } from 'ngx-spinner';
import { PublicationInfoComponent } from '../../components/publication/publication-info/publication-info.component';
import { MatDialog } from '@angular/material/dialog';
import { typeSearchCatalogue } from 'src/app/models/enums.model';
import { CollectionService, ISearchCommand as ISearchCommandCollection } from '../../domain/services/collection.service';
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

  private searchObjectPublication: ISearchCommandPublication = { status: 'published' };
  private searchObjectCollection: ISearchCommandCollection = {};

  public isSearched = false;
  public isSpinnerLoading = true;

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

    /* TODO: Deberia implementarse en catalogue-filter, dos formularios para mantener los estados que se
    habian colocado de los filtros (para no reiniciarlos)
    */
    this.searchObjectPublication =  { status: 'published'};
    this.searchObjectCollection = {};
    this.isSearched = false;

    this.isSpinnerLoading = true;

    ( this.currentTypeSearch === typeSearchCatalogue.publication  ) ?
      this.getPublicationData() :
      this.getCollectionData();

  }

  private getPublicationData(): void {

    this.publicationService.search( this.searchObjectPublication, 'category' ).subscribe(
      (searchRes) => {

        // console.log( 'Test > ', searchRes );
        this.publicationList = searchRes.items;
        this.spinnerService.hide();
        this.isSpinnerLoading = false;
      },
      (err: Error) =>  {

        console.error(err);
        this.spinnerService.hide();
        this.isSpinnerLoading = false;

      }
    );

  }

  private getCollectionData(): void {


    this.collectionService.search( this.searchObjectCollection ).subscribe(
      (searchRes) => {
        // console.log( 'Test > ', searchRes );
        this.collectionList = searchRes.items;
        this.spinnerService.hide();
        this.isSpinnerLoading = false;
      },
      (err: Error) =>  {

        console.error(err);
        this.spinnerService.hide();
        this.isSpinnerLoading = false;

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

  public onSearch( objectToSearch: any ): void {

    // console.log('TEST > '. objectToSearch );
    this.isSearched = true;
    this.spinnerService.show();
    this.isSpinnerLoading = true;
    if ( this.currentTypeSearch === this.optionTypeSearch.publication ) {

      this.searchObjectPublication.date_from = objectToSearch.dateFrom;
      this.searchObjectPublication.date_to = objectToSearch.dateTo;
      this.searchObjectPublication.order_by = objectToSearch.orderBy;
      this.searchObjectPublication.category_id  = objectToSearch.category_id;
      this.getPublicationData();

    } else {

      this.searchObjectCollection.date_from = objectToSearch.dateFrom;
      this.searchObjectCollection.date_to = objectToSearch.dateTo;
      this.searchObjectCollection.category_id = objectToSearch.category_id;
      this.getCollectionData();

    }

  }

  public goToObra( idObra: string ): void {

    this.router.navigate( [`/read/${idObra}`] );

  }

}
