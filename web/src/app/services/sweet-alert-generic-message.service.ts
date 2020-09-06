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

}
