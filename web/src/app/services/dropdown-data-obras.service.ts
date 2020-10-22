import { Injectable } from '@angular/core';
import { IDropdownItem } from '../models/dropdown-item.interface';
import { Observable, of } from 'rxjs';
import { CategoryService } from '../domain/services/category.service';
import { CollectionService, ISearchCommand } from '../domain/services/collection.service';
import { map } from 'rxjs/operators';

@Injectable({
  providedIn: 'root'
})
export class DropdownDataObrasService {

  constructor(
    private categoryService: CategoryService,
    private collectionService: CollectionService,
  ) { }

  public getAllCollectionDropdownDataById(idUser: string): Observable<IDropdownItem[]> {

    return this.collectionService.search({ author_id: idUser }, '').pipe(
      map(data => {
        return data.items.map(collection => {
          return {
            valueId: collection.id,
            name: collection.name,
          };
        });
      })
    );
  }

  public getAllCategoryDropdown(): Observable<IDropdownItem[]> {

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
