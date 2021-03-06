import { Component, Input, OnInit } from '@angular/core';
import { ICollection } from '../../../domain/models/collection';
import { faEdit, faInfoCircle } from '@fortawesome/free-solid-svg-icons';
import { ActivatedRoute, Router } from '@angular/router';
import { IUser, can } from 'src/app/domain/models';

@Component({
  selector: 'app-collection-card-author',
  templateUrl: './collection-card-author.component.html',
  styleUrls: ['./collection-card-author.component.scss']
})
export class CollectionCardAuthorComponent implements OnInit {

  @Input() collection: ICollection;
  @Input() userData: IUser;

  // Font Awseome icons
  public faEdit = faEdit;
  public faInfo = faInfoCircle;

  public can = can;

  constructor(
    private router: Router,
    private activatedRoute: ActivatedRoute
  ) { }

  ngOnInit(): void {
  }

  public onGoToEdit(): void {
    this.router.navigate([`collection/edit/${this.collection.id}`], { relativeTo: this.activatedRoute });
  }

  public onGoToCollectionInfo(): void {

    this.router.navigate([`collection/${this.collection.id}`]);

  }

}
