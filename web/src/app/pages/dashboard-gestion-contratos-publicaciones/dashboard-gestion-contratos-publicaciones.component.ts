import { Component, OnInit } from '@angular/core';

import { faSyncAlt, faBan, faCheckCircle } from '@fortawesome/free-solid-svg-icons';
import { IPublication } from '../../domain/models';
import { PublicationService } from '../../domain/services/publication.service';
import { NgxSpinnerService } from 'ngx-spinner';
import { MatDialog } from '@angular/material/dialog';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';
import { PublicationApproveRejectMotiveComponent } from 'src/app/components/dashboard/publication-approve-reject-motive/publication-approve-reject-motive.component';
import { BreakpointObserver } from '@angular/cdk/layout';
import { ContractService } from '../../domain/services/contract.service';
import { IContract } from '../../domain/models/contract';

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

  public selectedPanel = 'publication';

  public publicationList: IPublication[] = [];
  public contracts: IContract[] = [];
  public isBigScreen = true;

  constructor(
    private publicationService: PublicationService,
    private contractService: ContractService,
    private spinnerService: NgxSpinnerService,
    public dialog: MatDialog,
    private sweetAlertGenericService: SweetAlertGenericMessageService,
    private breakpointObserver: BreakpointObserver,
  ) { }

  ngOnInit(): void {

    this.checkWidthScreen();
    this.getAllPublication();

  }

  selectPanel(panel: string): void {
    this.selectedPanel = panel;

    if (panel === 'publication') {
      this.getAllPublication();
    } else if (panel === 'contract') {
      this.getAllContracts();
    }
  }


  public getAllPublication(): void {

    this.spinnerService.show();

    this.publicationService.search({ status: 'waiting-approval', order_by: 'newest' }, 'author,category').subscribe(
      (res) => {

        this.publicationList = res.items;
        // console.log(this.publicationList);

        this.spinnerService.hide();

      },
      (err: Error) => {

        console.error(err);
        this.spinnerService.hide();

      },
    );

  }

  public getAllContracts(): void {
    this.contractService.search({ status: 'requested' }, 'publication').subscribe(
      (res) => {
        this.contracts = res.items;
      },
      (err) => {
        console.log(err);
      },
    );
  }

  private checkWidthScreen(): void {

    this.breakpointObserver.observe(['(max-width: 950px)']).subscribe(
      (result: any) => {

        this.isBigScreen = (result.matches) ? false : true;

      });
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

  public approveContract(contract: IContract): void {
    this.contractService.approve(contract.id).subscribe(
      (res) => {
        this.getAllContracts();
      },
    );
  }

  public rejectContract(contract: IContract): void {
    this.contractService.reject(contract.id).subscribe(
      (res) => {
        this.getAllContracts();
      },
    );
  }

}
