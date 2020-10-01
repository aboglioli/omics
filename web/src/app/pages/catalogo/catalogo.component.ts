import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { PublicationService, ISearchResponse } from '../../domain/services/publication.service';
import { IPublication } from '../../domain/models/publication';
import { NgxSpinnerService } from 'ngx-spinner';
import { PublicationInfoComponent } from '../../components/publication/publication-info/publication-info.component';
import { MatDialog } from '@angular/material/dialog';

@Component({
  selector: 'app-catalogo',
  templateUrl: './catalogo.component.html',
  styleUrls: ['./catalogo.component.scss']
})
export class CatalogoComponent implements OnInit {

  publicationList: IPublication[] = [];


  constructor(
    private router: Router,
    private publicationService: PublicationService,
    private spinnerService: NgxSpinnerService,
    private dialog: MatDialog,
  ) { }

  ngOnInit(): void {

    this.spinnerService.show();
    this.publicationService.search( { status: 'published' } ).subscribe(
      (searchRes: ISearchResponse ) => {

        this.publicationList = searchRes.publications;
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
