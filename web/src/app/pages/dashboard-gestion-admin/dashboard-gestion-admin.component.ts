import { BreakpointObserver } from '@angular/cdk/layout';
import { Component, OnInit } from '@angular/core';
import { faClone, faTag } from '@fortawesome/free-solid-svg-icons';
import { DashboardGeneralOptionMenu } from 'src/app/models/enums.model';

@Component({
  selector: 'app-dashboard-gestion-admin',
  templateUrl: './dashboard-gestion-admin.component.html',
  styleUrls: ['./dashboard-gestion-admin.component.scss']
})
export class DashboardGestionAdminComponent implements OnInit {

  // FontAwesome Icon
  public faCategory = faClone;
  public faTag = faTag;


  public optionMenu = DashboardGeneralOptionMenu;
  public currentOption = this.optionMenu.categories; // TODO: En vez de esto, debería usarse "patch child" (esto lo hice por tiempo)

  public isBigScreen = true;

  constructor(
    private breakpointObserver: BreakpointObserver
  ) { }

  ngOnInit(): void {

    this.checkWidthScreen();

  }

  public onChangeTopMenu( option: number ): void {
    this.currentOption = option;
  }

  private checkWidthScreen(): void {

    this.breakpointObserver.observe(['(max-width: 950px)']).subscribe(
      (result: any) => {

        this.isBigScreen = (result.matches) ? false : true;

      });
  }

}
