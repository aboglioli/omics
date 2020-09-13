import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { MatDialog } from '@angular/material/dialog';
import { PasswordRewriteComponent } from '../../components/password-recovery/password-rewrite/password-rewrite.component';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {

  private paramsToUse: any;

  constructor(  private route: ActivatedRoute,
                private dialog: MatDialog ) { }

  ngOnInit(): void {

    if ( this.isRecoverPasswordNeeded() ) {

      this.showRecoverPasswrodModal();

    }

  }

  private isRecoverPasswordNeeded(): boolean {

    let isNeeded: boolean;

    this.route.params.subscribe( (params: any) => {

      isNeeded = (params.id) ? true : false;
      this.paramsToUse = params;

    });

    return isNeeded;

  }

  private showRecoverPasswrodModal(): void {

    const dialogRef = this.dialog.open(PasswordRewriteComponent, {
      data: {
              userId:  this.paramsToUse.id,
              temporalPass: this.paramsToUse.temporal_password,
              isRecoveryPassword: true
            },
      panelClass: 'no-padding-dialog'
    });

  }

}
