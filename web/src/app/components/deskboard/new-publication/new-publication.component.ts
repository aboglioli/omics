import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { AuthService } from '../../../domain/services/auth';
import { IPublication } from '../../../domain/models/publication';
import { FormGroup, FormBuilder, Validators } from '@angular/forms';

import { IDropdownItem } from '../../../models/dropdown-item.interface';


@Component({
  selector: 'app-new-publication',
  templateUrl: './new-publication.component.html',
  styleUrls: ['./new-publication.component.scss']
})
export class NewPublicationComponent implements OnInit {

  // Usados para Forms
  formPublication: FormGroup;
  publicationNewObject: IPublication;
  collectionList: IDropdownItem[] = [
    {
      valueId: 'id1',
      name: 'Colección 1'
    },
    {
      valueId: 'id2',
      name: 'Colección 2'
    },
    {
      valueId: 'id3',
      name: 'Colección 3'
    },
    {
      valueId: 'id4',
      name: 'Colección 4'
    },
    {
      valueId: 'id5',
      name: 'Colección 5'
    }
  ];

  // Otros
  ripplePortadaEnable = true;
  totalPages = 0;

  constructor(
    private router: Router,
    private authService: AuthService,
    private fb: FormBuilder,
  ) { }

  ngOnInit(): void {

    this.authService.authStart();
    this.buildForms();

  }

  public backToDeskboard(): void {

    this.router.navigate(['/deskboard'] );

  }

  private buildForms(): void {

    this.formPublication = this.fb.group({

      cover: ['', Validators.required ],
      name: ['', [ Validators.required, Validators.minLength(5) ] ],
      collection: [ '' ],
      synopsis: [ '', [ Validators.required, Validators.minLength(5) ] ],
      category_id: [ '', Validators.required ],
      tagstags: [ null ],
      pages: this.fb.array( this.buildPageForm() )

    });

  }

  private buildPageForm(): FormGroup[] {

    // TODO: Cuando esto sea para editar, hay que revisar que se completen con lo ya existente con un for

    return [this.fb.group({

      number: [ null ],
      images: [ '']

    })];

  }

  public uploadImagePortada(): void {

    console.log('TEST > ', 'Subir imagen');

  }

  public guardarBorrador(): void {

    console.log('TEST > Guardar en borrador');

  }

  public submitPublication(): void {

    console.log('TEST > Submit Publication > ', this.formPublication.value );

    if ( this.formPublication.invalid ) {

      return Object.values( this.formPublication.controls ).forEach( control => {

        // Si es un objeto
        if ( control instanceof FormGroup ) {

          Object.values( control.controls ).forEach( subControl => subControl.markAsTouched() );

        } else {

          control.markAsTouched(); // Marcar todos como tocadas

        }

      } );

    } else {

      console.log('TEST > PUBLICADO ');

    }

  }

  // Getters
  get nombreNovalido(): boolean {
    return ( this.formPublication.get('name').invalid && this.formPublication.get('name').touched );
  }

}
