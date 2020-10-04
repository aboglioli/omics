import { Component, OnInit } from '@angular/core';

import { faSyncAlt, faBan, faCheckCircle } from '@fortawesome/free-solid-svg-icons';
import { IPublication } from '../../domain/models';
import { PublicationService } from '../../domain/services/publication.service';
import { NgxSpinnerService } from 'ngx-spinner';
import { MatDialog } from '@angular/material/dialog';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';
import { PublicationApproveRejectMotiveComponent } from 'src/app/components/dashboard/publication-approve-reject-motive/publication-approve-reject-motive.component';

@Component({
  selector: 'app-dashboard-gestion-contratos-publicaciones',
  templateUrl: './dashboard-gestion-contratos-publicaciones.component.html',
  styleUrls: ['./dashboard-gestion-contratos-publicaciones.component.scss']
})
export class DashboardGestionContratosPublicacionesComponent implements OnInit {

  // FontAwesome Icon
  public faRefresh = faSyncAlt;
  public faReject = faBan;
  public faApprove = faCheckCircle;

  public publicationList: IPublication[];

  constructor(
    private publicationService: PublicationService,
    private spinnerService: NgxSpinnerService,
    public dialog: MatDialog,
    private sweetAlertGenericService: SweetAlertGenericMessageService,
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


  public openMessageReasonDialog( publication: IPublication, isApproved: boolean ): void {

    let reasonPublication: string;

    const dialogRef = this.dialog.open(PublicationApproveRejectMotiveComponent, {
      width: '300px',
      data: {
        approve: isApproved,
        publicationName: publication.name
      }
    });

    dialogRef.afterClosed().subscribe(result => {
      reasonPublication = result;

      if ( reasonPublication ) {

        ( isApproved ) ?
          this.approve(publication, reasonPublication) :
          this.reject( publication, reasonPublication );

      }

    });

  }


  private approve(publication: IPublication, commentReason: string): void {


    this.spinnerService.show();

    this.publicationService.approve( publication.id,  { comment: commentReason }).subscribe(
      (res: any) => {
        console.log(res);
        this.getAllPublication();
      },
      (err: Error) => {
        console.error(err);
        this.sweetAlertGenericService.showAlertError( `No se ha podido aprobar la publicación ${ publication.id }` );
        this.spinnerService.hide();
      }
    );

  }

  private reject(publication: IPublication, commentReason: string): void {

    this.spinnerService.show();

    this.publicationService.reject(publication.id, { comment: commentReason }).subscribe(
      (res: any) => {

        console.log(res);
        this.getAllPublication();

      },
      (err: Error) => {

        console.error(err);
        this.sweetAlertGenericService.showAlertError( `No se ha podido rechazar la publicación ${ publication.id }` );
        this.spinnerService.hide();

      }
    );
  }

}
