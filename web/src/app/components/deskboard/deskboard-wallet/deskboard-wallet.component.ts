import { Component, OnInit } from '@angular/core';
import { NgxSpinnerService } from 'ngx-spinner';

import { IContract } from '../../../domain/models/contract';
import { PublicationService } from '../../../domain/services/publication.service';
import { AuthorService } from '../../../domain/services/author.service';

@Component({
  selector: 'app-deskboard-wallet',
  templateUrl: './deskboard-wallet.component.html',
  styleUrls: ['./deskboard-wallet.component.scss']
})
export class DeskboardWalletComponent implements OnInit {
  private contracts: IContract[] = [];

  constructor(
    private authorService: AuthorService,
    private publicationService: PublicationService,
    private spinnerService: NgxSpinnerService,
  ) { }

  ngOnInit(): void {
    this.spinnerService.show();

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

}
