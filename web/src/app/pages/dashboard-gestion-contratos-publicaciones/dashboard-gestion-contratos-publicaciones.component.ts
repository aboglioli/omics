import { Component, OnInit } from '@angular/core';

import { faSyncAlt } from '@fortawesome/free-solid-svg-icons';
import { IPublication } from '../../domain/models';
import { PublicationService } from '../../domain/services/publication.service';
import { NgxSpinnerService } from 'ngx-spinner';


@Component({
  selector: 'app-dashboard-gestion-contratos-publicaciones',
  templateUrl: './dashboard-gestion-contratos-publicaciones.component.html',
  styleUrls: ['./dashboard-gestion-contratos-publicaciones.component.scss']
})
export class DashboardGestionContratosPublicacionesComponent implements OnInit {

  // FontAwesome Icon
  public faRefresh = faSyncAlt;

  public publicationList: IPublication[];

  constructor(
    private publicationService: PublicationService,
    private spinnerService: NgxSpinnerService,
  ) { }

  ngOnInit(): void {


    this.getAllPublication();

  }

  public getAllPublication(): void {

    this.spinnerService.show();

    this.publicationService.search({ status: 'waiting-approval' }, 'author,category').subscribe(
      (res: any) => {

        this.publicationList = res.publications;
        console.log(this.publicationList);

        this.spinnerService.hide();

      },
      (err: Error) => {

        console.error(err);
        this.spinnerService.hide();

      },
    );

  }

  approve(publication: IPublication): void {

    const commentPublication = 'comment...';  // TODO: sacar de un input de un popup
    // this.publicationService.approve( publication.id,  { comment: commentPublication }).subscribe(
    //   (res: any) => {
    //     console.log(res);
    //   },
    //   (err: Error) => {
    //     console.log(err);
    //   }
    // );

  }

  reject(publication: IPublication): void {

    const commentPublication = 'comment...'; // TODO: sacar de un input de un popup
    // this.publicationService.reject(publication.id, { comment: commentPublication }).subscribe(
    //   (res: any) => {
    //     console.log(res);
    //   },
    //   (err: Error) => {
    //     console.log(err);
    //   }
    // );
  }

}
