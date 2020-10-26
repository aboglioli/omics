import { Component, OnInit } from '@angular/core';
import { NgxSpinnerService } from 'ngx-spinner';

import { IContract } from '../../../domain/models/contract';
import { PublicationService } from '../../../domain/services/publication.service';
import { AuthorService } from '../../../domain/services/author.service';
import { ContractService } from '../../../domain/services/contract.service';
import Swal from 'sweetalert2';
import { DeskboardMedioCobroComponent } from '../deskboard/deskboard-medio-cobro.component';
import { MatDialog } from '@angular/material/dialog';
import { AuthService } from '../../../domain/services/auth.service';
@Component({
  selector: 'app-deskboard-wallet',
  templateUrl: './deskboard-wallet.component.html',
  styleUrls: ['./deskboard-wallet.component.scss']
})
export class DeskboardWalletComponent implements OnInit {
  public contracts: IContract[];
  public message: string;

  constructor(
    private authorService: AuthorService,
    private publicationService: PublicationService,
    private contractService: ContractService,
    private spinnerService: NgxSpinnerService,
    private authService: AuthService,
    private dialog: MatDialog,
  ) { }

  ngOnInit(): void {


    const userId = this.authService.getIdUser(); // Obtener el mail de medio de pago para modificar

    this.getContracts();
  }

  private getContracts(): void {
    this.spinnerService.show();

    this.contracts = [];

    this.authorService.getPublications('me').subscribe(
      (res) => {
        const publications = res.items
          .filter((p) => p.contract);

        for (const publication of publications) {
          this.publicationService.getContract(publication.id).subscribe(
            (res) => {
              this.contracts.push({
                ...res,
                publication,
              });
            },
          );
        }

        this.spinnerService.hide();
      },
    );
  }

  public generateSummaries(contract: IContract): void {
    this.spinnerService.show();

    this.publicationService.generateSummaries(contract.publication.id).subscribe(
      (res) => {
        this.getContracts();

        this.spinnerService.hide();
      },
    );
  }

  public totalAmount(contract: IContract): number {
    return contract
      .summaries
      .reduce((acc, s) => acc + s.amount, 0.0);
  }

  public paidAmount(contract: IContract): number {
    return contract
      .payments
      .reduce((acc, p) => acc + p.amount, 0.0);
  }

  public chargeAmount(contract: IContract): number {
    return contract
      .summaries
      .filter((s) => !s.paid)
      .reduce((acc, s) => acc + s.amount, 0.0);
  }

  public canCharge(contract: IContract): boolean {
    return contract.summaries.some((s) => !s.paid);
  }

  public charge(contract: IContract): void {
    this.spinnerService.show();

    this.contractService.charge(contract.id).subscribe(
      (res) => {
        this.getContracts();

        this.spinnerService.hide();
      }
    );
  }


  public onRescindirContrato( contract: IContract ): void {

    Swal.fire({
      icon: 'error',
      title: `¿Estas seguró rescindir el contrato de ${contract.publication.name}?`,
      showCancelButton: true,
      showConfirmButton: true,
      confirmButtonColor: 'red',
      confirmButtonText: 'Rescindir Contrato',
      cancelButtonText: 'Cancelar',
      focusCancel: true,
    }).then( result => {

      if ( result.isConfirmed ) {

        this.contractService.delete( contract.id ).subscribe(

          (res) => {

            this.getContracts();

          },
          (err: Error ) => {

            console.error(err);

          }

        );

      }

    } );
  }

  public onMedioCobro(): void {

    const mailCobro = '';

    const dialogRef = this.dialog.open(
      DeskboardMedioCobroComponent,
      {
        panelClass: 'margin-dialog',
        data: {
          mailCobro
        }
      }
    );

    // dialogRef.afterClosed().subscribe( resHaveEmail => {

    // });

  }

}
