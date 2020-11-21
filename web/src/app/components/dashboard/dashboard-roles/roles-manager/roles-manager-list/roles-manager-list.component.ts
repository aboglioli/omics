import { Component, OnInit } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { faPlusCircle } from '@fortawesome/free-solid-svg-icons';
import { RolesManagerEditComponent } from '../roles-manager-edit/roles-manager-edit.component';

// TODO: Borrar luego de aplicar servicio
const MOCKUP_DATA: any[] = [
  {id: '1', name: 'permiso 1', permisos: ['permiso 1']},
  {id: '2', name: 'permiso 2', permisos: ['permiso 2']},
  {id: '3', name: 'permiso 3', permisos: ['permiso 3']},
  {id: '4', name: 'permiso 4', permisos: ['permiso 4']},
  {id: '5', name: 'permiso 5', permisos: ['permiso 5']},
  {id: '6', name: 'permiso 6', permisos: ['permiso 6']},
  {id: '7', name: 'permiso 7', permisos: ['permiso 7']},
  {id: '8', name: 'permiso 8', permisos: ['permiso 8']},
  {id: '9', name: 'permiso 9', permisos: ['permiso 9']},
];

@Component({
  selector: 'app-roles-manager-list',
  templateUrl: './roles-manager-list.component.html',
  styleUrls: ['./roles-manager-list.component.scss']
})
export class RolesManagerListComponent implements OnInit {


  // Font Awseome icons
  public faAdd = faPlusCircle;

  // Información tablas
  tableRoleData: any[] = MOCKUP_DATA;
  displayedColumns: string[] = ['id', 'name', 'permisos'];

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
