import { Component, OnInit } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { faPlusCircle } from '@fortawesome/free-solid-svg-icons';
import { RolesManagerEditComponent } from '../roles-manager-edit/roles-manager-edit.component';

@Component({
  selector: 'app-roles-manager-list',
  templateUrl: './roles-manager-list.component.html',
  styleUrls: ['./roles-manager-list.component.scss']
})
export class RolesManagerListComponent implements OnInit {

  // Font Awseome icons
  public faAdd = faPlusCircle;

  constructor(
    private dialog: MatDialog,
  ) { }

  ngOnInit(): void {
  }

  onAddRol(): void {

    const dialogRef = this.dialog.open(
      RolesManagerEditComponent,
      {
        panelClass: 'info-publication',
        disableClose: true,
        data: {
          isNew: true,
        }
      }
    );

  }

}
