import { BreakpointObserver } from '@angular/cdk/layout';
import { Component, EventEmitter, OnInit, Output } from '@angular/core';
import { FormBuilder, FormGroup } from '@angular/forms';
import { typeSearchCatalogue } from '../../models/enums.model';

@Component({
  selector: 'app-catalogue-filter',
  templateUrl: './catalogue-filter.component.html',
  styleUrls: ['./catalogue-filter.component.scss']
})
export class CatalogueFilterComponent implements OnInit {

  @Output() typeSearch = new EventEmitter();
  @Output() filterSearch = new EventEmitter();

  public optionTypeSearch = typeSearchCatalogue;
  public currentTypeSearch = this.optionTypeSearch.publication;

  public isBigScreen = true;

  public publicationFilterTypesOrderList = [

    {
      value: 'most_viewed',
      name: 'Top Visualizaciones',
      selected: false,
    },
    {
      value: 'most_liked',
      name: 'Top Likes',
      selected: false,
    },
    {
      value: 'newest',
      name: 'Más nuevos',
      selected: false,
    },
    {
      value: 'best_reviews',
      name: 'Mejor calificado',
      selected: false,
    }

  ];


  // Del formulario
  public formFilterSearch: FormGroup;
  public maxDateToSerch: Date = new Date();

  constructor(
    private breakpointObserver: BreakpointObserver,
    private fb: FormBuilder,
  ) { }

  ngOnInit(): void {

    this.createForm();
    this.checkWidthScreen();

  }

  private createForm(): void {

    this.formFilterSearch = this.fb.group({
      name:          [],
      orderBy:       [''],
      category_id:   [''],
      dateFrom:      [],
      dateTo:        [],
    });


  }

  private resetForm(): void {
    this.formFilterSearch.reset();
  }

  public changeTypeSearch(type: typeSearchCatalogue): void {

    this.currentTypeSearch = type;
    this.resetForm();
    this.typeSearch.emit(this.currentTypeSearch);
  }

  private checkWidthScreen(): void {

    this.breakpointObserver.observe(['(max-width: 950px)']).subscribe(
      (result: any) => {

        this.isBigScreen = (result.matches) ? false : true;

      });
  }

  public convertDateToRFC3339(changeDate: Date, controlName: string): void {

    if ( controlName === 'dateTo'){

      changeDate = new Date(changeDate);
      changeDate.setHours( 23 );

    }
    this.formFilterSearch.get(controlName).setValue( changeDate.toISOString() );

  }

  public onFilterSelected( indexTypeFilter: number, isSelected: boolean, sortValue: string ): void {

    this.publicationFilterTypesOrderList.forEach( filter => {
      filter.selected = false;
    });

    this.publicationFilterTypesOrderList[indexTypeFilter].selected = isSelected;

    if ( isSelected  ) {
      this.formFilterSearch.get('orderBy').setValue( sortValue );
    } else {
      this.formFilterSearch.get('orderBy').setValue(null);
    }

    this.filterSearch.emit(  this.formFilterSearch.value );

  }

  public onSearchByDate(): void {


    if ( this.formFilterSearch.touched ) {
      this.filterSearch.emit(  this.formFilterSearch.value );
    }

  }


}
