import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { PublicationService, ISearchResponse } from '../../domain/services/publication.service';
import { IPublication } from '../../domain/models/publication';
import { ISearchCommand } from '../../domain/services/author.service';
import { NgxSpinnerService } from 'ngx-spinner';

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
  ) { }

  ngOnInit(): void {

    this.spinnerService.show();
    this.publicationService.search( { status: 'published' } ).subscribe(
      (searchRes: ISearchResponse ) => {

        this.publicationList = searchRes.publications;
        this.spinnerService.hide();
        console.log(this.publicationList);


      },
      (err: Error) =>  {

        console.error(err);
        this.spinnerService.hide();

      }
    );

  }

  public goToObra( idObra: string ): void {

    this.router.navigate( [`/read/${idObra}`] );

  }


}
