import { Component, OnInit, Input, Output } from '@angular/core';
import { IPublication } from 'src/app/domain/models';

@Component({
  selector: 'app-publication-card-reader',
  templateUrl: './publication-card-reader.component.html',
  styleUrls: ['./publication-card-reader.component.scss']
})
export class PublicationCardReaderComponent implements OnInit {

  @Input() publication: IPublication;

  constructor() { }

  ngOnInit(): void {
  }

}
