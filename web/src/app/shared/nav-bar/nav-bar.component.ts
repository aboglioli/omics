import { Component, OnInit, Output, EventEmitter } from '@angular/core';
import { faBars } from '@fortawesome/free-solid-svg-icons';
import { Router } from '@angular/router';
import { MatDialog } from '@angular/material/dialog';
import { LoginRegisterComponent } from '../../components/login-register/login-register.component';

@Component({
  selector: 'app-nav-bar',
  templateUrl: './nav-bar.component.html',
  styleUrls: ['./nav-bar.component.scss']
})
export class NavBarComponent implements OnInit {


  @Output() clickSideNavToggle = new EventEmitter();

  // Font Awseome icons
  public faBars = faBars;

  constructor(  private router: Router,
                public dialog: MatDialog ) { }

  ngOnInit(): void {
  }

  public toggleSideNavMenu(): void {

    this.clickSideNavToggle.emit();

  }

  public goToPage( pagePath: string ): void {

    this.router.navigate( ['/', pagePath] );

  }

  public openLoginRegisterDialog(): void {

    const dialogRef = this.dialog.open(LoginRegisterComponent);

  }


}
