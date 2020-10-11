import { Component, Input, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { ICollection } from '../../../domain/models/collection';

@Component({
  selector: 'app-collection-card-reader',
  templateUrl: './collection-card-reader.component.html',
  styleUrls: ['./collection-card-reader.component.scss']
})
export class CollectionCardReaderComponent implements OnInit {

  @Input() collection: ICollection;

  constructor(
    private router: Router,
  ) { }

  ngOnInit(): void {
  }

  public onGoToCollection(): void {

    this.router.navigate([`collection/${this.collection.id}`]);

  }

}
