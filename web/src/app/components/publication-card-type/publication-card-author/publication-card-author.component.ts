import { Component, OnInit, Input } from '@angular/core';
import { IPublication } from 'src/app/domain/models';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';
import { IStatus } from '../../../domain/models/publication';

@Component({
  selector: 'app-publication-card-author',
  templateUrl: './publication-card-author.component.html',
  styleUrls: ['./publication-card-author.component.scss']
})
export class PublicationCardAuthorComponent implements OnInit {

  @Input() publication: IPublication;

  public statusToShow = {
    msg: '',
    title: '',
    index: 0
  };

  constructor(
    private sweetAlertGenericService: SweetAlertGenericMessageService,
  ) { }

  ngOnInit(): void {

    this.setStatusCard( this.publication.status );

  }

  public goToEdit(): void {

    // TODO: Agregar para ver la publicación
    this.sweetAlertGenericService.showUnderConstrucction();

  }

  public setStatusCard( status: IStatus ): void {

    switch ( status.status ) {


      case 'waiting-approval': {

        this.statusToShow.title = 'PENDIENTE';
        this.statusToShow.msg = status.comment;
        this.statusToShow.index = 0;
        break;
      }

      case 'rejected': {

        this.statusToShow.title = 'RECHAZADO';
        this.statusToShow.msg = status.comment;
        this.statusToShow.index = 1;
        break;

      }

      case 'published': {

        this.statusToShow.title = 'PUBLICADO';
        this.statusToShow.msg = status.comment;
        this.statusToShow.index = 2;
        break;

      }

    }

  }

  public showStatusMsg(): void {

    switch (  this.statusToShow.index ) {

      case 0: {
        this.sweetAlertGenericService.showAlertInfo(  'Su publicación aún se encuentra en revisión', 'PENDIENTE' );
        break;
      }

      case 1: {
        this.sweetAlertGenericService.showAlertError(  this.statusToShow.msg, this.statusToShow.title );
        break;
      }

      case 2: {
        this.sweetAlertGenericService.showAlertSuccess(  this.statusToShow.msg, this.statusToShow.title );
        break;
      }

    }

  }

}
