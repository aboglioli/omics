import { Component, OnInit, Input, } from '@angular/core';
import { faEye, faHeart, faStar } from '@fortawesome/free-solid-svg-icons';
import { IPublication } from 'src/app/domain/models';

@Component({
  selector: 'app-publication-card-reader',
  templateUrl: './publication-card-reader.component.html',
  styleUrls: ['./publication-card-reader.component.scss']
})
export class PublicationCardReaderComponent implements OnInit {

  @Input() publication: IPublication;

  // Font Awseome icons
  public faVistas = faEye;
  public faLike = faHeart;
  public faStarFill = faStar;

  constructor() { }

  ngOnInit(): void {

    console.log(this.publication);
  }

}
