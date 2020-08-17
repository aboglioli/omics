import { Injectable } from '@angular/core';
import { FormGroup } from '@angular/forms';

@Injectable({
  providedIn: 'root'
})
export class ValidadoresCustomService {

  constructor() { }

    // Para comparar dos contraseñas si son iguales (recibiendo un formulario y devolviendo una función para comprobar asyncrono)
    passwordsIguales( pass1Name: string, pass2Name: string  ): ( (formGroup: FormGroup) => void ) {

      return( formGroup: FormGroup) => {

        const pass1Control = formGroup.controls[pass1Name];
        const pass2Control = formGroup.controls[pass2Name];

        if ( pass1Control.value === pass2Control.value ) {

          pass2Control.setErrors(null); // No devuelve error

        } else {

          pass2Control.setErrors( { noEsIgual: true } ); // Devuelve error

        }

      };

    }

}
