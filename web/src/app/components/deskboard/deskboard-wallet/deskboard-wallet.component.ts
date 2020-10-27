import { Component, OnInit } from '@angular/core';
import { NgxSpinnerService } from 'ngx-spinner';

import { IContract } from '../../../domain/models/contract';
import { IDonation } from '../../../domain/models/donation';
import { IBusinessRules } from '../../../domain/models/business-rules';
import { PublicationService } from '../../../domain/services/publication.service';
import { BusinessRulesService } from '../../../domain/services/business-rules.service';
import { AuthorService } from '../../../domain/services/author.service';
import { ContractService } from '../../../domain/services/contract.service';
import { DonationService } from '../../../domain/services/donation.service';
import Swal from 'sweetalert2';
import { DeskboardMedioCobroComponent } from '../deskboard/deskboard-medio-cobro.component';
import { MatDialog } from '@angular/material/dialog';
import { AuthService } from '../../../domain/services/auth.service';
import { IdentityService } from '../../../domain/services/identity.service';
import { IUser } from 'src/app/domain/models';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';
@Component({
  selector: 'app-deskboard-wallet',
  templateUrl: './deskboard-wallet.component.html',
  styleUrls: ['./deskboard-wallet.component.scss']
})
export class DeskboardWalletComponent implements OnInit {
  public contracts: IContract[] = [];
  public donations: IDonation[] = [];
  public businessRules: IBusinessRules;
  public message: string;

  private emailPaymentUser: string;

  constructor(
    private authorService: AuthorService,
    private publicationService: PublicationService,
    private contractService: ContractService,
    private donationService: DonationService,
    private businessRulesService: BusinessRulesService,
    private spinnerService: NgxSpinnerService,
    private authService: AuthService,
    private identifyService: IdentityService,
    private sweetAlertGenericService: SweetAlertGenericMessageService,
    private dialog: MatDialog,
  ) { }

  ngOnInit(): void {


    const userId = this.authService.getIdUser(); // Obtener el mail de medio de pago para modificar

    this.getBusinessRules();
    this.getContracts();
    this.getDonations();

  }

  private getBusinessRules(): void {
    this.businessRulesService.get().subscribe(
      (res) => {
        this.businessRules = res;
      },
    )
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

        this.getPayMentEmail();
      },
    );
  }

  private getDonations(): void {
    this.spinnerService.show();

    this.donations = [];

    this.authorService.getDonations('me', 'reader').subscribe(
      (res) => {
        this.donations = res.items;
      },
    );
  }


  private getPayMentEmail(): void {

    this.identifyService.getById( 'me' ).subscribe(
      (res: IUser) => {

        this.emailPaymentUser = res.payment_email;
        // console.log('test >', this.emailPaymentUser)
        this.spinnerService.hide();

      },
      ( err: Error ) => {

        console.error(err);
        this.spinnerService.hide();


      }
    );

  }

  public calculateDonationSubtotal(donation: IDonation): number {
    if (!this.businessRules) {
      return donation.amount;
    }

    const percentage = 1.0 - this.businessRules.donation_percentage_retention;
    return donation.amount * percentage;
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
    if (!this.businessRules) {
      return false;
    }

    return contract.summaries.some((s) => !s.paid) && this.chargeAmount(contract) >= this.businessRules.minimum_charge_amount;
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

    const dialogRef = this.dialog.open(
      DeskboardMedioCobroComponent,
      {
        panelClass: 'margin-dialog',
        data: {
          mailCobro: this.emailPaymentUser
        }
      }
    );

    dialogRef.afterClosed().subscribe( resEmail => {

      if ( resEmail ) {
        this.emailPaymentUser = resEmail;

        this.spinnerService.show();

        this.identifyService.changePaymentEmail( this.authService.getIdUser(), { payment_email: this.emailPaymentUser } ).subscribe(
          (res: any) => {

            this.sweetAlertGenericService.showAlertSuccess(`El correo actualmente vinculado al pago por Mercado Pago es: ${  this.emailPaymentUser }`, 'Email de cobro cambiado');
            this.spinnerService.hide();

          },
          (err: Error) => {

            console.error(err);

            this.spinnerService.hide();

          }
        );

      }


    });

  }

  public donationsTotalAmount(): number {
    return this.donations
      .reduce((acc, d) => {
        return acc + d.amount
      }, 0);
  }

  public donationsPaidAmount(): number {
    return this.donations
      .reduce((acc, d) => {
        if (!d.author_charge) {
          return acc;
        }

        return acc + d.amount
      }, 0);
  }

  public donationsChargeAmount(): number {
    return this.donations
      .reduce((acc, d) => {
        if (d.author_charge) {
          return acc;
        }

        return acc + d.amount
      }, 0);
  }

  public canChargeDonations(): boolean {
    if (!this.businessRules) {
      return false;
    }

    return this.donationsChargeAmount() >= this.businessRules.minimum_charge_amount;
  }

  public chargeDonations(): void {
    this.spinnerService.show();

    this.donationService.charge().subscribe(
      (res) => {
        this.getDonations();

        this.spinnerService.hide();
      },
    )
  }

}
