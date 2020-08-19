import { Component, OnInit } from '@angular/core';
import { ObrasService } from '../../services/obras.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-catalogo',
  templateUrl: './catalogo.component.html',
  styleUrls: ['./catalogo.component.scss']
})
export class CatalogoComponent implements OnInit {

  listaObras: any[] = [];


  constructor(
    private router: Router,
    private obrasService: ObrasService
  ) { }

  ngOnInit(): void {

    this.obrasService.getListaObras().subscribe( obras => {

      this.listaObras = obras;
      // console.log('test > ', obras);

    } );

  }

  public goToObra( idObra: string ): void {

    this.router.navigate( [`/read/${idObra}`] );

  }


}
