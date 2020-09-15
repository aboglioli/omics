import { Injectable } from '@angular/core';
import Swal from 'sweetalert2';

@Injectable({
  providedIn: 'root'
})
export class SweetAlertGenericMessageService {

  constructor() { }

  public showAlertError( msg: string, titleMsg: string = 'Error' ): void {

    Swal.fire({
      icon: 'error',
      title: titleMsg,
      text: msg
    });

  }

  public showAlertSuccess( msg: string, titleMsg: string = 'Éxito' ): void {

    Swal.fire({
      icon: 'success',
      title: titleMsg,
      text: msg
    });

  }

  public showAlertInfo( msg: string, titleMsg: string = 'Información' ): void {

    Swal.fire({
      icon: 'info',
      title: titleMsg,
      text: msg
    });

  }

  public showUnderConstrucction(): void {

    Swal.fire({
      icon: 'info',
      title: 'En construcción',
      text: 'Esta funcionalidad esta en desarrollo :)'
    });

  }


}
