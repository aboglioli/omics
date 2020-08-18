import { Component, OnInit } from '@angular/core';
import { faFileUpload } from '@fortawesome/free-solid-svg-icons';
import { Router, ActivatedRoute } from '@angular/router';

@Component({
  selector: 'app-deskboard-mis-comics',
  templateUrl: './deskboard-mis-comics.component.html',
  styleUrls: ['./deskboard-mis-comics.component.scss']
})
export class DeskboardMisComicsComponent implements OnInit {

  // Font Awseome icons
  public faUpload = faFileUpload;

  constructor(
    private router: Router,
    private route: ActivatedRoute
  ) { }

  ngOnInit(): void {
  }

  public goToNewPublication(): void {

    this.router.navigate(['publication/new'], { relativeTo: this.route });

  }

}
