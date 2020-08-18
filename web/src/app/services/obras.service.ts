import { Injectable } from '@angular/core';
import { Observable, of } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class ObrasService {

  mockDataObra: any = [

    {
      id: '1',
      thumbailUrl: 'https://via.placeholder.com/125'
    },
    {
      id: '2',
      thumbailUrl: 'https://via.placeholder.com/125'
    },
    {
      id: '3',
      thumbailUrl: 'https://via.placeholder.com/125'
    },


  ];

  constructor() { }

  public getListaObras(): Observable<any> {

    return of( this.mockDataObra );

  }

}
