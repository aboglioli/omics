import { Component, EventEmitter, OnInit, Output } from '@angular/core';
import { typeSearchCatalogue } from '../../models/enums.model';

@Component({
  selector: 'app-catalogue-filter',
  templateUrl: './catalogue-filter.component.html',
  styleUrls: ['./catalogue-filter.component.scss']
})
export class CatalogueFilterComponent implements OnInit {

  @Output() typeSearch = new EventEmitter();

  public optionTypeSearch = typeSearchCatalogue;
  public currentTypeSearch = this.optionTypeSearch.publication;

  constructor() { }

  ngOnInit(): void {
  }

  public changeTypeSearch(type: typeSearchCatalogue): void {

    this.currentTypeSearch = type;
    this.typeSearch.emit(this.currentTypeSearch);
  }


}
