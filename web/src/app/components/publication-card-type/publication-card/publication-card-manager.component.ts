import { Component, OnInit, Input } from '@angular/core';
import { IPublication } from '../../../domain/models/publication';
import { Router } from '@angular/router';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';



@Component({
  selector: 'app-publication-card',
  templateUrl: './publication-card-manager.component.html',
  styleUrls: ['./publication-card-manager.component.scss']
})
export class PublicationCardManagerComponent implements OnInit {
  @Input() publication: IPublication;

  constructor(
    private router: Router,
    private sweetAlertGenericService: SweetAlertGenericMessageService
  ) { }

  ngOnInit(): void {
    console.log(this.publication);
  }

  public goToProfile(): void {

    this.router.navigate( [`/profile/${this.publication.author.id}`]);

  }

  public goToPublication(): void {

    // TODO: Agregar para ver la publicación
    this.sweetAlertGenericService.showUnderConstrucction();

  }

}
