import { Component, OnInit } from '@angular/core';
import { typeManagueRoles } from 'src/app/models/enums.model';

import { AuthService } from '../../domain/services/auth.service';
import { IUser, can } from '../../domain/models/user';
import { SweetAlertGenericMessageService } from '../../services/sweet-alert-generic-message.service';

@Component({
  selector: 'app-dashboard-roles',
  templateUrl: './dashboard-roles.component.html',
  styleUrls: ['./dashboard-roles.component.scss']
})
export class DashboardRolesComponent implements OnInit {

  public optionMenu = typeManagueRoles;
  public currentOption = this.optionMenu.roles; // TODO: En vez de esto, debería usarse "patch child" (esto lo hice por tiempo)

  public userData: IUser;
  public showRoleOption: boolean;
  public can = can;

  constructor(
    private authService: AuthService,
  ) { }

  ngOnInit(): void {
    this.authService.getUser().subscribe((user) => {
      this.userData = user;
      if (can(user, 'get_any_role') && can(user, 'get_permissions')) {
        this.showRoleOption = true;
      } else {

        this.currentOption = this.optionMenu.users;
        this.showRoleOption = false;

      }
    });
  }

  public onSelectManageTypeMenu( option: number ): void {
    this.currentOption = option;
  }

}


