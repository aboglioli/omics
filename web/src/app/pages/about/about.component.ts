import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { faFacebook } from '@fortawesome/free-brands-svg-icons';

export interface IFaq {

  question: string;
  paragraph: string[];

}

export interface ITeamOmics {
  name: string;
  photo: string;
  url: string;
}
@Component({
  selector: 'app-about',
  templateUrl: './about.component.html',
  styleUrls: ['./about.component.scss']
})
export class AboutComponent implements OnInit {

  faqList: Array<IFaq>;

  teamList: Array<ITeamOmics> = [
    {
      name: 'Gustavo Lemos',
      photo: '/assets/images/team-photo/gustavo.jpg',
      url: 'https://twitter.com/TavoDandy'
    },
    {
      name: 'Daniela Amin',
      photo: '/assets/images/team-photo/daniela.jpg',
      url: ''
    },
    {
      name: 'Mariel Haarth',
      photo: '/assets/images/team-photo/mariel.jpg',
      url: 'https://twitter.com/H_Mariel'
    },
    {
      name: 'Alan Boglioli',
      photo: '/assets/images/team-photo/alan.jpg',
      url: 'https://twitter.com/alanboglioli'
    },
    {
      name: 'Julián Muñoz V.',
      photo: '/assets/images/team-photo/julian.jpg',
      url: 'https://twitter.com/Momfus'
    }
  ];

  // FontAwesome iconos
  public faFacebook = faFacebook;

  constructor(
    private httpClient: HttpClient
  ) { }

  ngOnInit(): void {

    this.httpClient.get('assets/json/faq.json').subscribe((data: any) => {
      this.faqList = data.faq;
    });

  }

  public onGoFacebookPage(): void {

    window.open( 'https://www.facebook.com/OmicsReader/', '_blank' );

  }

  public onGoToTweeter( dev: ITeamOmics ): void {

    if (  dev.url ) {

      window.open( dev.url, '_blank' );

    }

  }

}
