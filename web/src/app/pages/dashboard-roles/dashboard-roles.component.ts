import { Component, OnInit } from '@angular/core';
import { typeManagueRoles } from 'src/app/models/enums.model';

import { AuthService } from "../../domain/services/auth.service";
import { IUser, can } from "../../domain/models/user";

@Component({
  selector: 'app-dashboard-roles',
  templateUrl: './dashboard-roles.component.html',
  styleUrls: ['./dashboard-roles.component.scss']
})
export class DashboardRolesComponent implements OnInit {

  public optionMenu = typeManagueRoles;
  public currentOption = this.optionMenu.roles; // TODO: En vez de esto, debería usarse "patch child" (esto lo hice por tiempo)

  public user: IUser;
  public can = can;

  constructor(
    private authService: AuthService,
  ) { }

  ngOnInit(): void {
    this.authService.getUser().subscribe((user) => {
      this.user = user;
      if (!can(user, 'get_any_role')) {
        this.currentOption = this.optionMenu.users;
      }
    });
  }

  public onSelectManageTypeMenu( option: number ): void {
    this.currentOption = option;
  }

}


