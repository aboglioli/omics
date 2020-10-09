import { Component, Input, OnInit } from '@angular/core';
import { ICollection } from '../../../domain/models/collection';
import { faEdit, faInfoCircle } from '@fortawesome/free-solid-svg-icons';
import { ActivatedRoute, Router } from '@angular/router';
import { MatDialog } from '@angular/material/dialog';
import { CollectionInfoComponent } from '../../collection-info/collection-info.component';

@Component({
  selector: 'app-collection-card-author',
  templateUrl: './collection-card-author.component.html',
  styleUrls: ['./collection-card-author.component.scss']
})
export class CollectionCardAuthorComponent implements OnInit {

  @Input() collection: ICollection;

  // Font Awseome icons
  public faEdit = faEdit;
  public faInfo = faInfoCircle;

  constructor(
    private router: Router,
    private activatedRoute: ActivatedRoute,
    private dialog: MatDialog,
  ) { }

  ngOnInit(): void {
  }

  public onGoToEdit(): void {
    this.router.navigate([`collection/edit/${this.collection.id}`], { relativeTo: this.activatedRoute });
  }

  public onOpenInfo(): void {

    const dialogRef = this.dialog.open(
      CollectionInfoComponent,
      {
        panelClass: 'info-publication',
        data: {
          idCollection: this.collection.id
        }
      }
    );
  }

}
