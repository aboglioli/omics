import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';

import { ReaderService } from '../../domain/services/reader.service';
import { AuthService } from '../../domain/services/auth.service';
import { IPublication } from '../../domain/models';

@Component({
  selector: 'app-favoritos',
  templateUrl: './favoritos.component.html',
  styleUrls: ['./favoritos.component.scss']
})
export class FavoritosComponent implements OnInit {
  public publications: IPublication[];

  constructor(
    private readerService: ReaderService,
    private authService: AuthService,
    private router: Router,
  ) { }

  ngOnInit(): void {
    const readerId = this.authService.getIdUser();

    this.readerService.getFavorites(readerId, 'author,category').subscribe(
      res => {
        this.publications = res.publications;
      },
      err => {
        console.log(err);
      }
    );
  }

  public goToObra( idObra: string ): void {

    this.router.navigate( [`/read/${idObra}`] );

  }

}
