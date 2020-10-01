import { Component, OnInit, Input } from '@angular/core';
import { IPublication } from '../../../domain/models/publication';
import { Router } from '@angular/router';
import { PublicationInfoComponent } from '../../publication/publication-info/publication-info.component';
import { MatDialog } from '@angular/material/dialog';


@Component({
  selector: 'app-publication-card-manager',
  templateUrl: './publication-card-manager.component.html',
  styleUrls: ['./publication-card-manager.component.scss']
})
export class PublicationCardManagerComponent implements OnInit {

  @Input() publication: IPublication;

  constructor(
    private router: Router,
    private dialog: MatDialog,
  ) { }

  ngOnInit(): void {}

  public goToProfile(): void {

    this.router.navigate( [`/profile/${this.publication.author.id}`]);

  }

  public onOpenInfo(): void {

    const dialogRef = this.dialog.open(
      PublicationInfoComponent,
      {
        panelClass: 'info-publication',
        data: {
          idPublication: this.publication.id,
          showRead: true
        }
      }
    );

  }

}
