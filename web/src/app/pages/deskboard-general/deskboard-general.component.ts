import { Component, OnInit } from '@angular/core';
import { AuthService } from 'src/app/domain/services/auth.service';

@Component({
  selector: 'app-deskboard-general',
  templateUrl: './deskboard-general.component.html',
  styleUrls: ['./deskboard-general.component.scss']
})
export class DeskboardGeneralComponent implements OnInit {

  constructor(
    private authService: AuthService
  ) { }

  ngOnInit(): void {

    this.authService.authStart();

  }

}
