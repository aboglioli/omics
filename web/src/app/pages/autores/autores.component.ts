import { Component, OnInit } from '@angular/core';
import { faUsers } from '@fortawesome/free-solid-svg-icons';

import { IAuthor } from '../../domain/models';
import { AuthorService } from '../../domain/services/author.service';

@Component({
  selector: 'app-autores',
  templateUrl: './autores.component.html',
  styleUrls: ['./autores.component.scss']
})
export class AutoresComponent implements OnInit {

  // Font Awseome icons
  public faFollowers = faUsers;

  public authors: IAuthor[];

  constructor(
    private authorService: AuthorService,
  ) { }

  ngOnInit(): void {
    this.authorService.search({order_by: 'newest' }).subscribe(
      res => {
        this.authors = res.items;
      },
      err => {
        console.log(err);
      },
    );
  }

}
