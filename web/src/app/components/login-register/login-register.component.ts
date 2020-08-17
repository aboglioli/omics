import { Component, OnInit } from '@angular/core';
import { faTimesCircle } from '@fortawesome/free-solid-svg-icons';
import { FormGroup, FormBuilder } from '@angular/forms';
import { MatDialogRef } from '@angular/material/dialog';

@Component({
  selector: 'app-login-register',
  templateUrl: './login-register.component.html',
  styleUrls: ['./login-register.component.scss']
})
export class LoginRegisterComponent implements OnInit {

  // Font Awseome icons
  public faClose = faTimesCircle;

  constructor( private dialogRef: MatDialogRef<LoginRegisterComponent> ) {

    dialogRef.disableClose = true;

  }

  ngOnInit(): void {
  }

  public closeMatDialog(): void {



  }

}
