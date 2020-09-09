import { Component, OnInit } from '@angular/core';
import { IdentityService } from '../../domain/services/identity.service';

@Component({
  selector: 'app-perfil',
  templateUrl: './perfil.component.html',
  styleUrls: ['./perfil.component.scss']
})
export class PerfilComponent implements OnInit {

  constructor(
    private identifyService: IdentityService
  ) { }

  ngOnInit(): void {
  }

}
