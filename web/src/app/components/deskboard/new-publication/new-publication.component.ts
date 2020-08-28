import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { forkJoin } from 'rxjs';
import { FormGroup, FormBuilder, Validators, FormArray, FormControl } from '@angular/forms';
import { MatCheckbox } from '@angular/material/checkbox';
import {COMMA, ENTER} from '@angular/cdk/keycodes';

import { AuthService } from '../../../domain/services/auth';
import { DropdownDataObrasService } from '../../../services/dropdown-data-obras.service';
import { NgxSpinnerService } from 'ngx-spinner';

import { IPublication, ITag } from '../../../domain/models/publication';
import { IDropdownItem } from '../../../models/dropdown-item.interface';
import { MatChipInputEvent } from '@angular/material/chips';
import { throwToolbarMixedModesError } from '@angular/material/toolbar';


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
  categoryList: IDropdownItem[];
  tagsList: ITag[] = [];

  // Otros
  ripplePortadaEnable = true;
  totalPages = 0;

  chipTagsKeysCodes: number[] = [ENTER, COMMA]; // Usado para los tags

  constructor(
    private router: Router,
    private authService: AuthService,
    private fb: FormBuilder,
    private spinnerService: NgxSpinnerService,
    private dropdownDataObrasService: DropdownDataObrasService,
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
      synopsis: [ '', [ Validators.required, Validators.minLength(5),  Validators.maxLength(512) ] ],
      category_id: [ '', Validators.required ],
      tags: [ null ],
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
    setTimeout(() => {
      this.spinnerService.hide();
    }, 5000);

    const observableList =  [ this.dropdownDataObrasService.getAllCollectionDropdownDataById(),
                              this.dropdownDataObrasService.getAllCategoryDropdown()
                            ];

    forkJoin( observableList).subscribe(([ dataCollection, dataCategory ]) => {

      this.collectionList = dataCollection;
      this.categoryList = dataCategory;

      this.spinnerService.hide();


      });

  }

  public uploadImagePortada(): void {

    // Crear elemento input de tipo 'file' para poder manejarlo desde el botón que lo llama
    const inputFileElement = document.createElement('input');
    inputFileElement.type = 'file'; // Nota:  Solo uno a la vez, para varios: inputFileElement.multiple = multiple
    inputFileElement.accept = '.png, .jpg, .jpeg';
    inputFileElement.click();

    // Definir la función del llamado al hacer click (cuando realiza un cambio)
    inputFileElement.onchange = ( event: any ) => {

      const fdImage: FormData = new FormData();
      const imagePortada  = event.target.files[0];

      // #region Cargar para previsualizar en pantalla

      const reader = new FileReader();
      reader.onload = (eventReader: any ) => {

        this.portadaImage = eventReader.target.result;

      };

      reader.readAsDataURL(imagePortada);

      //#endregion

      // #region Generar un nombre para enviar el archivo
      let imageName =  imagePortada.lastModified + imagePortada.name;
      imageName = imageName.replace(/\s+/g, '-').toLowerCase();
      imageName = imageName.substr(0, imageName.lastIndexOf('.'));
      // #endregion

      fdImage.append('image', imagePortada, imageName);
      this.formPublication.get('cover').setValue(fdImage);
      // console.log('TEST > ', imagePortada );
      // console.log('TEST > ', fdImage.getAll('image') );

    };

  }

  public guardarBorrador(): void {

    console.log('TEST > Guardar en borrador');

  }

  public submitPublication(): void {

    this.formPublication.get('tags').setValue(this.tagsList);

    // Reducir descripción los espacios vacios que pueda tener al final
    const description = this.formPublication.get('synopsis');
    this.formPublication.get('synopsis').setValue(description.value.trim());

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

  // #region Tags

  public addTag( event: MatChipInputEvent): void {

    const input = event.input;
    const value = event.value.trim();

    if ((value || '')) {

      this.tagsList.push({
        id: value.replace(/\s+/g, '-').toLowerCase(),
        name: value
      });

    }

    // Reset the input value
    if (input) {
      input.value = '';
    }
  }

  public removeTag( tag: ITag ): void {

    const index = this.tagsList.indexOf(tag);

    if (index >= 0) {
      this.tagsList.splice(index, 1);
    }

  }

  // #endregion

  // Getters
  get nombreNovalido(): boolean {
    return ( this.formPublication.get('name').invalid && this.formPublication.get('name').touched );
  }

  get nombreSynopsis(): boolean {
    return ( this.formPublication.get('synopsis').invalid && this.formPublication.get('synopsis').touched );
  }


  get collectionArrayCheck(): FormArray {
    return this.formPublication.get('collectionArray') as FormArray;
  }


}
