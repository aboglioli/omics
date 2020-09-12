import { Component, OnInit } from '@angular/core';

import { IPublication } from '../../domain/models';
import { PublicationService } from '../../domain/services/publication.service';

@Component({
  selector: 'app-dashboard-gestion-contratos-publicaciones',
  templateUrl: './dashboard-gestion-contratos-publicaciones.component.html',
  styleUrls: ['./dashboard-gestion-contratos-publicaciones.component.scss']
})
export class DashboardGestionContratosPublicacionesComponent implements OnInit {
  public publications: IPublication[];

  constructor(
    private publicationService: PublicationService,
  ) { }

  ngOnInit(): void {
    this.publicationService.search({ status: 'waiting-approval' }, 'author,category').subscribe(
      (res) => {
        this.publications = res.publications;
        console.log(this.publications);
      },
      (err) => {
        console.log(err);
      },
    );
  }

  approve(publication: IPublication): void {
    this.publicationService.approve(
      publication.id,
      {
        comment: 'comment...' // TODO: take from input
      }
    ).subscribe(
      res => {
        console.log(res);
      },
      err => {
        console.log(err);
      }
    );
  }

  reject(publication: IPublication): void {
    this.publicationService.reject(
      publication.id,
      {
        comment: 'comment...' // TODO: take from input
      }
    ).subscribe(
      res => {
        console.log(res);
      },
      err => {
        console.log(err);
      }
    );
  }

}
