import { Component, OnInit, Output, EventEmitter } from '@angular/core';
import { faBars } from '@fortawesome/free-solid-svg-icons';
import { Router } from '@angular/router';

@Component({
  selector: 'app-nav-bar',
  templateUrl: './nav-bar.component.html',
  styleUrls: ['./nav-bar.component.scss']
})
export class NavBarComponent implements OnInit {


  @Output() clickSideNavToggle = new EventEmitter();

  // Font Awseome icons
  public faBars = faBars;

  constructor(  private router: Router ) { }

  ngOnInit(): void {
  }

  public toggleSideNavMenu(): void {

    this.clickSideNavToggle.emit();

  }

  public goToPage( pagePath: string ): void {

    this.router.navigate( ['/', pagePath] );

  }


}
