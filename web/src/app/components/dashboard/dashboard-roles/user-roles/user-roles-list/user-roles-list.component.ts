import { AfterViewInit, Component, OnInit, ViewChild } from '@angular/core';
import { MatPaginator, PageEvent } from '@angular/material/paginator';
import { MatTableDataSource } from '@angular/material/table';
import { NgxSpinnerService } from 'ngx-spinner';
import { IdentityService, ISearchCommand } from '../../../../../domain/services/identity.service';
import { IUser, IRole } from '../../../../../domain/models/user';
import { MatDialog } from '@angular/material/dialog';
import { UserRolesEditComponent } from '../user-roles-edit/user-roles-edit.component';
import { RoleService } from '../../../../../domain/services/role.service';

@Component({
  selector: 'app-user-roles-list',
  templateUrl: './user-roles-list.component.html',
  styleUrls: ['./user-roles-list.component.scss']
})
export class UserRolesListComponent implements OnInit, AfterViewInit {

  @ViewChild(MatPaginator, {static: false}) paginator: MatPaginator;

  public pageCurrent = 0;
  public pageLength = 0;
  public pageSize = 10;

  private searchUserCMD: ISearchCommand = {
    limit: 10,
    offset: this.pageCurrent,
    order_by: 'newest',
    name: '',
    role_id: '',
  };

  displayedColumns: string[] = ['username', 'name', 'role', 'created_at', 'validated'];
  tableUserData = new MatTableDataSource<IUser>();

  rolesAllData: IRole[];

  constructor(
    private spinnerService: NgxSpinnerService,
    private identityService: IdentityService,
    private dialog: MatDialog,
    private roleService: RoleService
  ) { }

  ngOnInit(): void {

    this.getAllUserData(true);
  }


  ngAfterViewInit(): void {
    this.tableUserData.paginator = this.paginator;
  }


  private getAllUserData( isRoleGetNeeded ): void {

    this.spinnerService.show();

    this.identityService.search(this.searchUserCMD, 'role').subscribe(
      (resUserList) => {

        // console.log( 'TEST > ', resUserList );

        this.tableUserData = new MatTableDataSource<IUser>(resUserList.items);
        this.pageLength = Math.ceil( resUserList.matching_criteria / this.pageSize );

        if ( isRoleGetNeeded ) {

          this.roleService.getAll().subscribe(
            (resRoleData) => {

              this.rolesAllData = resRoleData.roles;
              this.spinnerService.hide();

            }
          );

        } else {
          this.spinnerService.hide();
        }

      },
      (err: Error) => {
        this.spinnerService.hide();
        console.error(err);
      }
    );

  }

  public changePageOptions( pageEvent: PageEvent ): void {

    this.searchUserCMD.limit = pageEvent.pageSize;
    this.searchUserCMD.offset = pageEvent.pageIndex;

    console.log('TEST > ', pageEvent)
    this.getAllUserData(false);

  }

  public onEditUserRole( userRow: IUser ): void {

    const dialogRef = this.dialog.open(
      UserRolesEditComponent,
      {
        panelClass: 'info-publication',
        disableClose: true,

        data: {
          user: userRow,
          roleList: this.rolesAllData
        }
      }
    ).afterClosed().subscribe(result => {

        if ( result ) {
          this.getAllUserData( false );
        }

      }
    );

  }

  public filterUser( searchFilter: ISearchCommand ): void {

    console.log('TEST > ', searchFilter);

    this.searchUserCMD.name = searchFilter.name;
    this.searchUserCMD.role_id = searchFilter.role_id;
    this.searchUserCMD.offset = 0;
    this.pageCurrent = 0;
    this.paginator.firstPage();

    this.getAllUserData(false);

  }

}
