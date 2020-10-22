import { Component, OnInit } from '@angular/core';
import { NgxSpinnerService } from 'ngx-spinner';

import { IPublication } from '../../../domain/models/publication';
import { PublicationService } from '../../../domain/services/publication.service';
import { AuthorService } from '../../../domain/services/author.service';

@Component({
  selector: 'app-deskboard-report',
  templateUrl: './deskboard-report.component.html',
  styleUrls: ['./deskboard-report.component.scss']
})
export class DeskboardReportComponent implements OnInit {
  public publications: IPublication[] = [];

  constructor(
    private authorService: AuthorService,
    private publicationService: PublicationService,
    private spinnerService: NgxSpinnerService,
  ) { }

  ngOnInit(): void {
    this.getPublications();
  }

  private getPublications(): void {
    this.spinnerService.show();

    this.publications = [];

    const dateTo = new Date();
    const dateFrom = new Date();
    dateFrom.setHours(dateFrom.getHours() - 24 * 7);

    this.authorService.getPublications('me').subscribe(
      (res) => {
        const publications = res.items
          .filter((p) => p.status.status === 'published');

        for (const publication of publications) {
          this.publicationService
            .getStatistics(publication.id, {
              date_from: dateFrom.toISOString(),
              date_to: dateTo.toISOString(),
            }).subscribe(
              (res) => {
                const statistics = {
                  views: res.views || 0,
                  unique_views: res.unique_views || 0,
                  readings: res.readings || 0,
                  likes: res.likes || 0,
                  reviews: res.reviews || 0,
                  stars: res.stars || 0.0,
                };

                this.publications.push({
                  ...publication,
                  statistics,
                });
              },
            );
        }

        this.spinnerService.hide();
      },
    )
  }

}
