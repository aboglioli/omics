import { Component, Inject, Input, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { faTrashAlt, faSave, faTimesCircle, faChevronRight, faChevronLeft } from '@fortawesome/free-solid-svg-icons';
import { IPermission, IRole } from '../../../../../domain/models/user';

export interface DialogData {
  isNew: boolean;
  role: IRole;
  permissionArrayToSelect: IPermission[];
}

@Component({
  selector: 'app-roles-manager-edit',
  templateUrl: './roles-manager-edit.component.html',
  styleUrls: ['./roles-manager-edit.component.scss']
})
export class RolesManagerEditComponent implements OnInit {

  @Input() permissionArrayToSelect: Array<IPermission>;

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

  public currentItemSelected: any = null;
  public currentIndexElementSelected = 0;
  public isToRemoveArrow = false;

  constructor(
    @Inject(MAT_DIALOG_DATA) public data: DialogData,
    public dialogRef: MatDialogRef<RolesManagerEditComponent>,
    private fb: FormBuilder,
  ) {
    this.permissionArrayToSelect = this.data.permissionArrayToSelect;
  }

  ngOnInit(): void {


    this.isNewRole =  (this.data.isNew) ? true : false;
    this.formBuild();

    if ( this.isNewRole ) {

      this.title = 'Crear nuevo rol';

    } else {

      this.title = 'Editar rol';
      console.log('TEst > ', this.data.role)
      this.isDefault = this.data.role.default;
      this.setFormByData( this.data.role );

    }

    this.permissionArrayAssigned = this.data.role.permissions;
    this.permissionArrayAssigned.sort( (a, b) => a.name.localeCompare(b.name));

    // Quitar de los disponibles para asignar los que ya estan asignados
    const availables = this.permissionArrayToSelect.filter(p => {
      return this.permissionArrayAssigned.every(rp => rp.id !== p.id);
    });

    this.permissionArrayToSelect = availables;


  }

  private formBuild(): void {

    this.formRole = this.fb.group({
      name: [ '', [ Validators.required, Validators.minLength(4) ] ],
      permissionList: [ null ],
    });

  }

  private setFormByData( role: IRole ): void {
    this.formRole.reset({

      name: role.name,
      permissionList: role.permissions

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
