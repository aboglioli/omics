import { Component, OnInit } from '@angular/core';
import { NgxSpinnerService } from 'ngx-spinner';

import { IContract } from '../../../domain/models/contract';
import { PublicationService } from '../../../domain/services/publication.service';
import { AuthorService } from '../../../domain/services/author.service';
import { ContractService } from '../../../domain/services/contract.service';

@Component({
  selector: 'app-deskboard-wallet',
  templateUrl: './deskboard-wallet.component.html',
  styleUrls: ['./deskboard-wallet.component.scss']
})
export class DeskboardWalletComponent implements OnInit {
  public contracts: IContract[];

  constructor(
    private authorService: AuthorService,
    private publicationService: PublicationService,
    private contractService: ContractService,
    private spinnerService: NgxSpinnerService,
  ) { }

  ngOnInit(): void {
    this.getContracts();
  }

  private getContracts(): void {
    this.spinnerService.show();

    this.contracts = [];

    this.authorService.getPublications('me').subscribe(
      (res) => {
        const publications = res.publications
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

  generateSummaries(contract: IContract): void {
    this.spinnerService.show();

    this.publicationService.generateSummaries(contract.publication.id).subscribe(
      (res) => {
        this.getContracts();

        this.spinnerService.hide();
      },
    );
  }

  canCharge(contract: IContract): boolean {
    return contract.summaries.some((s) => !s.paid);
  }

  charge(contract: IContract): void {
    this.spinnerService.show();

    this.contractService.charge(contract.id).subscribe(
      (res) => {
        this.getContracts();

        this.spinnerService.hide();
      }
    );
  }

}
