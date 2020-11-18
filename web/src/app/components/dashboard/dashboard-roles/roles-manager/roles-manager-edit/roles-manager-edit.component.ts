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

  constructor(
    @Inject(MAT_DIALOG_DATA) public data: DialogData,
    public dialogRef: MatDialogRef<RolesManagerEditComponent>,
    private fb: FormBuilder,
  ) { }

  ngOnInit(): void {

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
      permission: [ null ],
    });

  }

  public onPermissionToAssign( indexItem: number ): void {

    const itemToAssign = this.permissionArrayToSelect[indexItem];

    console.log('TEST Assign > ', itemToAssign);


  }

  public onPermissionToRemove( indexItem: number ): void {

    const itemToRemove = this.permissionArrayToSelect[indexItem];

    console.log('TEST Remove > ', itemToRemove);


  }

  public deleteRol(): void {

  }

  public onSubmitForm(): void {

  }

  public onClose(): void {
    this.dialogRef.close();
  }

  // Getters
  get nombreNovalido(): boolean {
    return ( this.formRole.get('name').invalid && this.formRole.get('name').touched );
  }

}
