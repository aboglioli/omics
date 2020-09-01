import { Component, OnInit } from '@angular/core';

import { IdentityService, IRegisterCommand, } from '../../domain/services/identity.service';

@Component({
  selector: 'app-development',
  templateUrl: './development.component.html',
})
export class DevelopmentComponent implements OnInit {

  constructor(private identityServ: IdentityService) { }

  ngOnInit(): void {

  }

}
