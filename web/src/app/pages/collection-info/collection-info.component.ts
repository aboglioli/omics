import { Component, OnInit } from '@angular/core';
import { NgxSpinnerService } from 'ngx-spinner';
import { ActivatedRoute, Router } from '@angular/router';
import { CollectionService, IGetPublicationsResponse } from '../../domain/services/collection.service';
import { ICollection } from '../../domain/models/collection';
import { IPublication } from 'src/app/domain/models/publication';
import { MatDialog } from '@angular/material/dialog';
import { PublicationInfoComponent } from 'src/app/components/publication/publication-info/publication-info.component';

@Component({
  selector: 'app-collection-info',
  templateUrl: './collection-info.component.html',
  styleUrls: ['./collection-info.component.scss']
})
export class CollectionInfoComponent implements OnInit {

  public collectionData: ICollection;
  public publicationList: IPublication[];

  constructor(
    private spinnerService: NgxSpinnerService,
    private activatedRoute: ActivatedRoute,
    private collectionService: CollectionService,
    private dialog: MatDialog,
    private router: Router,
  ) { }

  ngOnInit(): void {

    this.getCollectionDataByParams();

  }

  private getCollectionDataByParams(): void {

    this.spinnerService.show();

    this.activatedRoute.params.subscribe(
      (params) => {

        this.collectionService.getById(params.id, 'author, category').subscribe(
          ( resData: ICollection) => {
            console.log(resData);

            this.collectionData = resData;
            this.getPublicationsByCategory( this.collectionData.id );
          },
          (err: Error) => {
            console.error(err);
            this.spinnerService.hide();
          }
        );
      }
    );

  }


  private getPublicationsByCategory( categoryId: string ): void {

    this.collectionService.getPublications( categoryId, 'category' ).subscribe(
      (resPublication: IGetPublicationsResponse) => {
        this.publicationList = resPublication.publications;

        this.spinnerService.hide();

      },
      (err: Error ) => {

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

  public onGoToAuthorProfile(): void {
    this.router.navigate( [`/profile/${this.collectionData.author.id}`] );
  }

}
