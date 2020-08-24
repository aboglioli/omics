import { Injectable } from '@angular/core';
import { IDropdownItem } from '../models/dropdown-item.interface';
import { Observable, of } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class CollectionFilterService {

  //#region Datos mock

  mockCollectionDropdownData: IDropdownItem[] = [

    {
      valueId: 'id1',
      name: 'Colección 1'
    },
    {
      valueId: 'id2',
      name: 'Colección 2'
    },
    {
      valueId: 'id3',
      name: 'Colección 3'
    },
    {
      valueId: 'id4',
      name: 'Colección 4'
    },
    {
      valueId: 'id5',
      name: 'Colección 5'
    }

  ];

  //#endregion

  constructor() { }

  public getCollectionDropdownData(): Observable<IDropdownItem[]> {

    return of( this.mockCollectionDropdownData );

  }

}
