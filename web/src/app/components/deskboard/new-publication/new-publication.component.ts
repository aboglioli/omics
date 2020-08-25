import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { AuthService } from '../../../domain/services/auth';
import { IPublication } from '../../../domain/models/publication';
import { FormGroup, FormBuilder, Validators, FormArray, FormControl } from '@angular/forms';

import { IDropdownItem } from '../../../models/dropdown-item.interface';
import { CollectionFilterService } from '../../../services/collections.service';
import { NgxSpinnerService } from 'ngx-spinner';
import { MatCheckbox } from '@angular/material/checkbox';


@Component({
  selector: 'app-new-publication',
  templateUrl: './new-publication.component.html',
  styleUrls: ['./new-publication.component.scss']
})
export class NewPublicationComponent implements OnInit {

  // Usados para Forms
  formPublication: FormGroup;
  publicationNewObject: IPublication;
  collectionList: IDropdownItem[];
  portadaImage = null;

  // Otros
  ripplePortadaEnable = true;
  totalPages = 0;

  constructor(
    private router: Router,
    private authService: AuthService,
    private fb: FormBuilder,
    private spinnerService: NgxSpinnerService,
    private collectionFilterService: CollectionFilterService,
  ) { }

  ngOnInit(): void {

    this.authService.authStart();
    this.buildForms();
    this.setSubscriptionData();

  }

  public backToDeskboard(): void {

    this.router.navigate(['/deskboard'] );

  }

  private buildForms(): void {

    this.formPublication = this.fb.group({

      cover: ['', Validators.required ],
      name: ['', [ Validators.required, Validators.minLength(5) ] ],
      collectionArray: this.fb.array([]),
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
      images: ['']

    })];

  }

  public setSubscriptionData(): void {

    this.spinnerService.show();

    this.collectionFilterService.getCollectionDropdownData().subscribe(  data => {

      this.collectionList = data;
      this.spinnerService.hide();

    });

  }

  public uploadImagePortada(): void {

    console.log('TEST > ', 'Subir imagen');

    // Crear elemento input de tipo 'file' para poder manejarlo desde el botón que lo llama
    const inputFileElement = document.createElement('input');
    inputFileElement.type = 'file'; // Nota:  Solo uno a la vez, para varios: inputFileElement.multiple = multiple
    inputFileElement.accept = '.png, .jpg, .jpeg';
    inputFileElement.click();

    // Definir la función del llamado al hacer click (cuando realiza un cambio)
    inputFileElement.onchange = ( event: any ) => {

      const fdImage: FormData = new FormData();
      const imagePortada  = event.target.files[0];


      //#region Cargar para previsualizar en pantalla

      const reader = new FileReader();
      reader.onload = (eventReader: any ) => {

        this.portadaImage = eventReader.target.result;

      };

      reader.readAsDataURL(imagePortada);

      //#endregion

      fdImage.append('image', imagePortada, 'testName');

      // this.portadaImage = event.target.files[0];
      const fileToUpload: File = null;

      console.log(imagePortada );
      console.log(fdImage );

    };

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


  // #region Dropdown Checkbox Collection

  public onCheckboxChangeCollection( event: MatCheckbox ): void {

    // Comprobar si debe agregarlo a lista o no
    if (  event.checked ) {

      this.collectionArrayCheck.push( new FormControl( event.value ) );

    } else {

      // Busca en todo el array el elemento que tiene el mismo valor que el que se saco para quitarlo del array
      let i = 0;
      this.collectionArrayCheck.controls.forEach( (item: FormControl) => {

        if ( item.value === event.value ) {

          this.collectionArrayCheck.removeAt(i);
          return;

        }

        i++;

      });

    }

  }

  public onRadioChangeCollection(): void {

    this.collectionArrayCheck.clear();

  }

  public isNotCheckedAllCollection(): boolean {

    return (this.collectionArrayCheck.length === 0);

  }

  public isCheckedCollectionItem( item: IDropdownItem ): boolean {

    return( (this.collectionArrayCheck.value as Array<string> ).indexOf(item.valueId) > -1 );

  }

  // #endregion

  // Getters
  get nombreNovalido(): boolean {
    return ( this.formPublication.get('name').invalid && this.formPublication.get('name').touched );
  }

  get collectionArrayCheck(): FormArray {
    return this.formPublication.get('collectionArray') as FormArray;
  }


}
