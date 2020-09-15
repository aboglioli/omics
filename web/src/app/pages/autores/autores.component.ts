import { Component, OnInit } from '@angular/core';

import { IAuthor } from '../../domain/models';
import { AuthorService } from '../../domain/services/author.service';

@Component({
  selector: 'app-autores',
  templateUrl: './autores.component.html',
  styleUrls: ['./autores.component.scss']
})
export class AutoresComponent implements OnInit {
  public authors: IAuthor[];

  constructor(
    private authorService: AuthorService,
  ) { }

  ngOnInit(): void {
    this.authorService.search({}).subscribe(
      res => {
        this.authors = res.authors.filter(author => !!author.profile_image);
      },
      err => {
        console.log(err);
      },
    )
  }

}
