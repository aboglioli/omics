import { Component, OnInit } from '@angular/core';
import { DashboardGeneralOptionMenu } from 'src/app/models/enums.model';

@Component({
  selector: 'app-dashboard-gestion-admin',
  templateUrl: './dashboard-gestion-admin.component.html',
  styleUrls: ['./dashboard-gestion-admin.component.scss']
})
export class DashboardGestionAdminComponent implements OnInit {


  public optionMenu = DashboardGeneralOptionMenu;
  public currentOption = this.optionMenu.categories; // TODO: En vez de esto, debería usarse "patch child" (esto lo hice por tiempo)

  constructor() { }

  ngOnInit(): void {
  }

}
