import { Component, OnInit } from '@angular/core';
import { faFileUpload } from '@fortawesome/free-solid-svg-icons';
import { Router, ActivatedRoute } from '@angular/router';
import { AuthorService } from '../../../domain/services/author.service';
import { NgxSpinnerService } from 'ngx-spinner';
import { IGetPublicationsResponse } from 'src/app/domain/services/collection.service';
import { IPublication } from '../../../domain/models/publication';

@Component({
  selector: 'app-deskboard-mis-comics',
  templateUrl: './deskboard-mis-comics.component.html',
  styleUrls: ['./deskboard-mis-comics.component.scss']
})
export class DeskboardMisComicsComponent implements OnInit {

  // Font Awseome icons
  public faUpload = faFileUpload;

  public publicationList: IPublication[];

  constructor(
    private router: Router,
    private activatedRoute: ActivatedRoute,
    private authorService: AuthorService,
    private spinnerService: NgxSpinnerService,
  ) { }

  ngOnInit(): void {

    this.spinnerService.show();
    this.authorService.getPublications('me').subscribe(
      (resData: IGetPublicationsResponse) => {

        this.publicationList = resData.publications;
        console.log(this.publicationList);

        this.spinnerService.hide();

      },
      (err: Error) => {

        console.error(err);
        this.spinnerService.hide();

      }
    )

  }

  public goToNewPublication(): void {

    this.router.navigate(['publication/new'], { relativeTo: this.activatedRoute });

  }

}
