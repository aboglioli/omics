import { Component, OnInit, Input } from '@angular/core';
import { IPublication } from '../../../domain/models/publication';



@Component({
  selector: 'app-publication-card',
  templateUrl: './publication-card-manager.component.html',
  styleUrls: ['./publication-card-manager.component.scss']
})
export class PublicationCardManagerComponent implements OnInit {
  @Input()
  publication: IPublication;

  constructor() { }

  ngOnInit(): void {
    console.log(this.publication);
  }

}
