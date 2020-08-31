import { Injectable } from '@angular/core';
import { IDropdownItem } from '../models/dropdown-item.interface';
import { Observable, of } from 'rxjs';
import { CategoryService } from '../domain/services/category';
import { map } from 'rxjs/operators';

@Injectable({
  providedIn: 'root'
})
export class DropdownDataObrasService {

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

  constructor(
    private categoryService: CategoryService
  ) { }

  // TODO: Cuando este el servicio en collection.ts el método para conseguir todas las colecciones de un autor, agregar parámetro
  public getAllCollectionDropdownDataById(): Observable<IDropdownItem[]> {

    return of( this.mockCollectionDropdownData );

  }

  public getAllCategoryDropdown():  Observable<IDropdownItem[]> {

    return this.categoryService.getAll().pipe(
      map( data => {
        return data.categories.map( category => { // No confundir este map de arrays con el anterior (de Rxjs)
          return {
            valueId: category.id,
            name: category.name
          };
        });
      })
    );

  }

}
