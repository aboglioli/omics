import { Component, OnInit, Input, ViewChild } from '@angular/core';
import { IReview } from '../../../domain/models/review';
import { IReaderPublicationInteraction } from '../../../domain/models/reader';
import { faChevronLeft, faChevronRight, faStar } from '@fortawesome/free-solid-svg-icons';
import { faStar as faStarAlt } from '@fortawesome/free-regular-svg-icons';
import { NgbCarousel } from '@ng-bootstrap/ng-bootstrap';

@Component({
  selector: 'app-publication-reviews-box',
  templateUrl: './publication-reviews-box.component.html',
  styleUrls: ['./publication-reviews-box.component.scss']
})
export class PublicationReviewsBoxComponent implements OnInit {

  @Input() reviewArray: IReview[];
  @Input() readerData: IReaderPublicationInteraction;

  @ViewChild('commentCarousel', { static: false }) carrouselComment: NgbCarousel;

  // Font Awseome icons
  public faLeft = faChevronLeft;
  public faRight = faChevronRight;
  public faStarFill = faStar;
  public faStarEmpty = faStarAlt;

  constructor() { }

  ngOnInit(): void {
  }

  public onPrevious(): void {

    this.carrouselComment.prev();

  }

  public onNext(): void {
    this.carrouselComment.next();
  }

}
