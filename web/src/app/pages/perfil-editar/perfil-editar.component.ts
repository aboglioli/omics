import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'app-perfil-editar',
  templateUrl: './perfil-editar.component.html',
  styleUrls: ['./perfil-editar.component.scss']
})
export class PerfilEditarComponent implements OnInit {

  userName: any;

  constructor(
    private activatedRoute: ActivatedRoute,
  ) {


  }

  ngOnInit(): void {


    this.userName =  this.activatedRoute.snapshot.paramMap.get('id');
    // this.setUserDataToEdit();

  }

}
