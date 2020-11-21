import { Component, Inject, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { faTrashAlt, faSave, faTimesCircle, faChevronRight, faChevronLeft } from '@fortawesome/free-solid-svg-icons';

export interface DialogData {
  isNew: boolean;
}

@Component({
  selector: 'app-roles-manager-edit',
  templateUrl: './roles-manager-edit.component.html',
  styleUrls: ['./roles-manager-edit.component.scss']
})
export class RolesManagerEditComponent implements OnInit {

  // FontAwesome Icon
  public faDelete = faTrashAlt;
  public faSave = faSave;
  public faClose = faTimesCircle;
  public faLeft = faChevronLeft;
  public faRight = faChevronRight;

  public isNewRole = false;
  public title = '';
  public isDefault = false;

  public formRole: FormGroup;

  public permissionArrayAssigned: Array<any> = [];

  public permissionArrayToSelect: Array<any> = [
    { id: '0', name: 'permiso 0' },
    { id: '1', name: 'permiso 1' },
    { id: '2', name: 'permiso 2' },
    { id: '3', name: 'permiso 3' },
    { id: '4', name: 'permiso 4' },
    { id: '5', name: 'permiso 5' },
    { id: '6', name: 'permiso 6' },
    { id: '7', name: 'permiso 7' },
    { id: '8', name: 'permiso 8' },
    { id: '9', name: 'permiso 9' },
  ];

  public currentItemSelected: any = null;
  public currentIndexElementSelected = 0;
  public isToRemoveArrow = false;

  constructor(
    @Inject(MAT_DIALOG_DATA) public data: DialogData,
    public dialogRef: MatDialogRef<RolesManagerEditComponent>,
    private fb: FormBuilder,
  ) { }

  ngOnInit(): void {

    // Quitar de los disponibles para asignar los que ya estan asignados
    const availables = this.permissionArrayToSelect.filter(p => {
      return this.permissionArrayAssigned.every(rp => rp.id !== p.id);
    });

    this.permissionArrayToSelect = availables;

    this.isNewRole =  (this.data.isNew) ? true : false;
    this.formBuild();

    if ( this.isNewRole ) {

      this.title = 'Crear nuevo rol';

    } else {

      this.title = 'Editar rol';
      // TODO: Asignar los datos a editar

    }

  }

  private formBuild(): void {

    this.formRole = this.fb.group({
      name: [ '', [ Validators.required, Validators.minLength(4) ] ],
      permissionList: [ null ],
    });

  }

  public onPermissionToMove( indexItem: number, isToRemove: boolean ): void {

    this.isToRemoveArrow = isToRemove;
    this.currentItemSelected = (isToRemove) ?
                                  this.permissionArrayAssigned[indexItem] :
                                  this.permissionArrayToSelect[indexItem];
    this.currentIndexElementSelected = indexItem;

  }

  public movePermission(): void {

    if ( this.isToRemoveArrow ) {

      this.removePermission();
      this.permissionArrayToSelect.sort( (a, b) => a.name.localeCompare(b.name));

    } else {

      this.assignPermission();
      this.permissionArrayAssigned.sort( (a, b) => a.name.localeCompare(b.name));

    }

  }

  private assignPermission(): void {

    this.permissionArrayAssigned.push( this.currentItemSelected );
    this.permissionArrayToSelect.splice(this.currentIndexElementSelected, 1);

    this.currentItemSelected = undefined;
    this.currentIndexElementSelected = null;

  }

  private removePermission(): void {

    this.permissionArrayToSelect.push( this.currentItemSelected );
    this.permissionArrayAssigned.splice(this.currentIndexElementSelected, 1);

    this.currentItemSelected = undefined;
    this.currentIndexElementSelected = null;

  }

  public deleteRol(): void {

  }

  public onSubmitForm(): void {

    const arrayPermissionId: Array<string> = new Array();

    this.permissionArrayAssigned.forEach( permission => {

      arrayPermissionId.push(permission.id);

    });

    this.formRole.get('permissionList').setValue(arrayPermissionId);

    if ( this.isDefault ) {
      // TODO: setear rol como predeterminado
    }

    console.log('TEST > ', this.formRole.value);
    console.log('TEST > ', this.isDefault);

  }

  public onClose(): void {
    this.dialogRef.close();
  }

  // Getters
  get nombreNovalido(): boolean {
    return ( this.formRole.get('name').invalid && this.formRole.get('name').touched );
  }

}
