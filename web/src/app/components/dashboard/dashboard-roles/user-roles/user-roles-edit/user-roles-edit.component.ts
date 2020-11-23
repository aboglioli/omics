import { Component, Inject, OnInit } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { Router } from '@angular/router';
import { faTimesCircle, faTrashAlt, faSave } from '@fortawesome/free-solid-svg-icons';
import { NgxSpinnerService } from 'ngx-spinner';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';
import Swal from 'sweetalert2';
import { IUser, IRole, can } from '../../../../../domain/models/user';
import { IdentityService } from '../../../../../domain/services/identity.service';
import { AuthService } from '../../../../../domain/services/auth.service';

export interface DialogData {
  user: IUser;
  roleList: IRole[];
}

@Component({
  selector: 'app-user-roles-edit',
  templateUrl: './user-roles-edit.component.html',
  styleUrls: ['./user-roles-edit.component.scss']
})
export class UserRolesEditComponent implements OnInit {

  // FontAwesome Icon
  public faDelete = faTrashAlt;
  public faSave = faSave;
  public faClose = faTimesCircle;

  public userData: IUser;
  public newRole: string;
  public roleListToSelect: IRole[];

  public authUser: IUser;
  public can = can;

  constructor(
    @Inject(MAT_DIALOG_DATA) public data: DialogData,
    public dialogRef: MatDialogRef<UserRolesEditComponent>,
    private router: Router,
    private identityService: IdentityService,
    private spinnerService: NgxSpinnerService,
    private sweetAlertGenericService: SweetAlertGenericMessageService,
    private authService: AuthService,
  ) { }

  ngOnInit(): void {

    // console.log('TEST > User > ', this.data.user );
    // console.log('TEST > Role List > ', this.data.roleList );

    this.userData = this.data.user;
    this.newRole = this.userData.role.id;
    this.roleListToSelect = this.data.roleList;

    this.authService.getUser().subscribe((user) => {
      this.authUser = user;
    });

  }

  public onClose(): void {
    this.dialogRef.close( false);
  }

  public onGoToUserProfile(): void {

    this.router.navigate( [`/profile/${this.userData.id}`] );
    this.onClose();

  }

  public deleteUser(): void {

    Swal.fire({
      title: `Eliminar usuario: ${this.userData.username}`,
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

        this.deleteUserConfirm();

      }
    });


  }

  private deleteUserConfirm(): void {

    this.spinnerService.show();
    this.identityService.delete(this.userData.id).subscribe(
      (res) => {
        this.spinnerService.hide();
        this.dialogRef.close(true);

        this.sweetAlertGenericService.showAlertSuccess(`El usuario ${ this.userData.username } ha sido eliminado correctamente.`, 'Eliminación exitosa');

      },
      (err ) => {
        this.spinnerService.hide();
        console.error(err);
      }
    );
  }

  public onChangeUserRole(): void {

    this.spinnerService.show();
    this.identityService.changeRole( this.userData.id, { role_id: this.newRole } ).subscribe(
      (res) => {

        this.spinnerService.hide();
        this.sweetAlertGenericService.showAlertSuccess(
          `El usuario ${this.userData.username} tiene ahora el rol ${this.newRole}`,
          'Rol asignado'
        );

        this.dialogRef.close(true);

      },
      (err) => {
        this.spinnerService.hide();
        console.error(err);
      }

    );

  }

}
