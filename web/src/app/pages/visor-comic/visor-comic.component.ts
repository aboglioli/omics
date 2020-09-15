import { Component, OnInit } from '@angular/core';
import { NgxSpinnerService } from 'ngx-spinner';
import { PublicationService } from 'src/app/domain/services/publication.service';
import { IPublication, IPage } from '../../domain/models/publication';
import { IGetByIdResponse, IReadResponse } from '../../domain/services/publication.service';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'app-visor-comic',
  templateUrl: './visor-comic.component.html',
  styleUrls: ['./visor-comic.component.scss']
})
export class VisorComicComponent implements OnInit {

  publicationToShow: IPublication;
  pagesList: IPage[];

  constructor(
    private spinnerService: NgxSpinnerService,
    private publicationService: PublicationService,
    private activateRoute: ActivatedRoute,
  ) { }

  ngOnInit(): void {
    this.getPublicationDataByParams();
  }

  private getPublicationDataByParams(): void {

    this.spinnerService.show();
    this.activateRoute.params.subscribe( params => {

      this.publicationService.getById(params.id).subscribe(

        (resPub: IGetByIdResponse) => {

          this.publicationToShow = resPub.publication;

          this.publicationService.read( params.id ).subscribe(

            (resPages: IReadResponse) => {

              this.pagesList = resPages.pages;
              console.log(this.pagesList);
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

}
