import { Component, OnInit } from '@angular/core';
import { typeManagueRoles } from 'src/app/models/enums.model';

@Component({
  selector: 'app-dashboard-roles',
  templateUrl: './dashboard-roles.component.html',
  styleUrls: ['./dashboard-roles.component.scss']
})
export class DashboardRolesComponent implements OnInit {

  public optionMenu = typeManagueRoles;
  public currentOption = this.optionMenu.roles; // TODO: En vez de esto, debería usarse "patch child" (esto lo hice por tiempo)

  constructor() { }

  ngOnInit(): void {
  }

  public onSelectManageTypeMenu( option: number ): void {
    this.currentOption = option;
  }

}


