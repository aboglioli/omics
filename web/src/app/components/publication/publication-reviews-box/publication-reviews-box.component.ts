import { Component, OnInit, Input } from '@angular/core';
import { IReview } from '../../../domain/models/review';
import { IReaderPublicationInteraction } from '../../../domain/models/reader';

@Component({
  selector: 'app-publication-reviews-box',
  templateUrl: './publication-reviews-box.component.html',
  styleUrls: ['./publication-reviews-box.component.scss']
})
export class PublicationReviewsBoxComponent implements OnInit {

  @Input() reviewArray: IReview[];
  @Input() readerData: IReaderPublicationInteraction;

  constructor() { }

  ngOnInit(): void {
  }

}
