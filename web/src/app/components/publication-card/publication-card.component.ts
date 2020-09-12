import { Component, OnInit, Input } from '@angular/core';

import { IPublication } from '../../domain/models';

@Component({
  selector: 'app-publication-card',
  templateUrl: './publication-card.component.html',
  styleUrls: ['./publication-card.component.scss']
})
export class PublicationCardComponent implements OnInit {
  @Input()
  publication: IPublication;

  constructor() { }

  ngOnInit(): void {
    console.log(this.publication);
  }

}
