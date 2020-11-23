import { Component, Inject, Input, OnInit, ɵConsole } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { faTrashAlt, faSave, faTimesCircle, faChevronRight, faChevronLeft } from '@fortawesome/free-solid-svg-icons';
import { NgxSpinnerService } from 'ngx-spinner';
import Swal from 'sweetalert2';
import { IPermission, IRole } from '../../../../../domain/models/user';
import { RoleService, ICreateCommand, IUpdateCommand } from '../../../../../domain/services/role.service';
import { SweetAlertGenericMessageService } from '../../../../../services/sweet-alert-generic-message.service';
import { AuthService } from '../../../../../domain/services/auth.service';
import { IUser, can } from '../../../../../domain/models/user';

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

  public user: IUser;
  public can = can;

  constructor(
    @Inject(MAT_DIALOG_DATA) public data: DialogData,
    public dialogRef: MatDialogRef<RolesManagerEditComponent>,
    private fb: FormBuilder,
    private roleService: RoleService,
    private spinnerService: NgxSpinnerService,
    private sweetAlertGenericService: SweetAlertGenericMessageService,
    private authService: AuthService,
  ) {
    this.permissionArrayToSelect = this.data.permissionArrayToSelect;
    this.permissionArrayToSelect.sort( (a, b) => a.name.localeCompare(b.name));
  }

  ngOnInit(): void {
    this.authService.getUser().subscribe((user) => {
      this.user = user;
    });

    this.isNewRole =  (this.data.isNew) ? true : false;
    this.formBuild();

    if ( this.isNewRole ) {

      this.title = 'Crear nuevo rol';

    } else {

      // console.log('TEst > ', this.data.role);
      this.title = 'Editar rol';
      this.isDefault = this.data.role.default;
      this.setFormByData( this.data.role );

    }

    this.permissionArrayAssigned = (this.data.role) ? this.data.role.permissions : [];
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
      permissionList: [],
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

    Swal.fire({
      title: `Eliminar Rol: ${this.data.role.name}`,
      text: '¿Estas seguro?',
      icon: 'warning',
      showCancelButton: true,
      focusCancel: true,
      confirmButtonColor: '#FC4850',
      cancelButtonColor: '#3085d6',
      confirmButtonText: 'Sí, eliminar',
      cancelButtonText: 'No'
    }).then((result) => {
      if (result.isConfirmed) {

        this.deleteRoleConfirm();

      }
    });


  }

  private deleteRoleConfirm(): void {

    this.spinnerService.show();
    this.roleService.delete(this.data.role.id).subscribe(
      (res) => {
        this.spinnerService.hide();
        this.dialogRef.close(true);

        this.sweetAlertGenericService.showAlertSuccess(`El rol ${ this.data.role.name } ha sido eliminado correctamente.`, 'Eliminación exitosa');

      },
      (err ) => {
        this.spinnerService.hide();

        switch ( err.error.code ){

          case 'existing_users_assigned_to_role': {
            this.sweetAlertGenericService.showAlertError(
              // tslint:disable-next-line: max-line-length
              'El rol no puede eliminarse debido a que hay usuarios asignados con el mismo. Contactarse con el encargado de la base de datos.',
              `No puede eliminarse ${ this.data.role.name }`
            );

            break;

          }

          case 'is_default': {

            this.sweetAlertGenericService.showAlertError('No puede eliminarse un rol preterminado');

            break;

          }

          default: {
            console.error(err);
          }

        }
      }
    );

  }

  public onSubmitForm(): void {

    const arrayPermissionId: Array<string> = new Array();

    this.permissionArrayAssigned.forEach( permission => {

      arrayPermissionId.push(permission.id);

    });

    this.formRole.get('permissionList').setValue(arrayPermissionId);

    if ( this.isNewRole ) {
      this.createRole();
    } else {
      this.editRole();
    }

  }

  private createRole(): void {

    const createdRole: ICreateCommand =  {
      name: this.formRole.get('name').value,
      permissions: this.formRole.get('permissionList').value
    };

    this.spinnerService.show();
    this.roleService.create( createdRole ).subscribe(
      ( resCreate ) => {

        this.spinnerService.hide();

        if ( this.isDefault ) {

          this.roleService.makeDefault( resCreate.id ).subscribe(
            (res) => {
              this.sweetAlertGenericService.showAlertSuccess(`El rol predeterminado ${ createdRole.name } ha sido creado correctamente`, 'Creación exitosa');
              this.dialogRef.close( true );
            }
          );

        } else {

          this.sweetAlertGenericService.showAlertSuccess(`El rol ${ createdRole.name } ha sido creado correctamente.`, 'Creación exitosa');
          this.dialogRef.close( true );

        }


      },
      ( err ) => {
        this.spinnerService.hide();

        if ( err.error.code === 'already_exists' ){

          this.sweetAlertGenericService.showAlertError(`El rol ${ createdRole.name } genera un id igual que otro rol existente`, 'ID repetido');

        } else {
          console.error('Error: ', err);
        }
      }
    );

  }

  private editRole(): void {

    const editedRole: IUpdateCommand = {
      name: this.formRole.get('name').value,
      permissions: this.formRole.get('permissionList').value
    };

    this.spinnerService.show();
    this.roleService.update( this.data.role.id, editedRole ).subscribe(
      (resEdited) => {
        this.spinnerService.hide();

        if ( this.isDefault ) {

          this.roleService.makeDefault( this.data.role.id ).subscribe(
            (res) => {
              this.sweetAlertGenericService.showAlertSuccess(`El rol predeterminado ${ editedRole.name } ha sido editado correctamente.`, 'Edición exitosa');
              this.dialogRef.close( true );
            }
          );

        } else {

          this.sweetAlertGenericService.showAlertSuccess(`El rol ${ editedRole.name } ha sido editado correctamente.`, 'Edición exitosa');
          this.dialogRef.close( true );

        }


        this.authService.loadUser();
      },
      (err ) => {

        if ( err.error.code === 'already_exists' ){
          this.sweetAlertGenericService.showAlertError(`El rol ${ editedRole.name } genera un id igual que otro rol existente`, 'ID repetido');
        } else {
          console.error('Error: ', err);
        }

      }
    );

  }

  public onClose(): void {
    this.dialogRef.close( false);
  }

  // Getters
  get nombreNovalido(): boolean {
    return ( this.formRole.get('name').invalid && this.formRole.get('name').touched );
  }

}
