import { Component, OnInit, ViewChild } from '@angular/core';
import { Router } from '@angular/router';
import { forkJoin, Observable, concat } from 'rxjs';
import { FormGroup, FormBuilder, Validators, FormArray, FormControl } from '@angular/forms';
import { MatCheckbox } from '@angular/material/checkbox';
import {COMMA, ENTER} from '@angular/cdk/keycodes';

import { faPlusCircle, faTimesCircle, faBookOpen } from '@fortawesome/free-solid-svg-icons';

import { AuthService } from '../../../domain/services/auth.service';
import { FileService } from '../../../domain/services/file.service';
import { DropdownDataObrasService } from '../../../services/dropdown-data-obras.service';
import { NgxSpinnerService } from 'ngx-spinner';

import { IPublication } from '../../../domain/models/publication';
import { IDropdownItem } from '../../../models/dropdown-item.interface';
import { MatChipInputEvent } from '@angular/material/chips';
import { SwalComponent } from '@sweetalert2/ngx-sweetalert2';
import { PublicationService, IUpdatePagesCommand } from '../../../domain/services/publication.service';
import { ICreateCommand } from '../../../domain/services/collection.service';


@Component({
  selector: 'app-new-publication',
  templateUrl: './new-publication.component.html',
  styleUrls: ['./new-publication.component.scss']
})
export class NewPublicationComponent implements OnInit {

  @ViewChild('formDataInvalid') private swalFormDataInvalid: SwalComponent;
  @ViewChild('formDataValid') private swalFormDataValid: SwalComponent;

  // FontAwesome Icon
  public faPlus = faPlusCircle;
  public faCloseCircle = faTimesCircle;
  public faBoookOpen = faBookOpen;

  // Usados para Forms
  public formPublication: FormGroup;
  public publicationNewObject: IPublication;
  public collectionList: IDropdownItem[];
  public portadaImage = {
    thumbail: null,
    url: null
  };
  public categoryList: IDropdownItem[];
  public tagsList: string[] = [];


  // Otros
  public ripplePortadaEnable = true;
  pageList: IUpdatePagesCommand;

  public chipTagsKeysCodes: number[] = [ENTER, COMMA]; // Usado para los tags

  constructor(
    private router: Router,
    private authService: AuthService,
    private fb: FormBuilder,
    private spinnerService: NgxSpinnerService,
    private dropdownDataObrasService: DropdownDataObrasService,
    private fileServ: FileService,
    private publicationService: PublicationService
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

      name: ['', [ Validators.required, Validators.minLength(5) ] ],
      cover: ['', Validators.required ],
      collectionArray: this.fb.array([]),
      synopsis: [ '', [ Validators.required, Validators.minLength(5),  Validators.maxLength(512) ] ],
      category_id: [ '', Validators.required ],
      tags: [ null ],
      pagesList: this.fb.array([])

    });

  }

  public setSubscriptionData(): void {

    this.spinnerService.show();
    setTimeout(() => {
      this.spinnerService.hide();
    }, 20000); // 20 segundos de espera máxima TODO: Agregar mensaje de error de pasar mucho tiempo

    const observableList =  [
        this.dropdownDataObrasService.getAllCollectionDropdownDataById(),
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

      this.spinnerService.show();

      // #region Cargar para previsualizar en pantalla

      const reader = new FileReader();
      let tempCoverThumbail;
      reader.onload = (eventReader: any ) => {

        tempCoverThumbail = eventReader.target.result;

      };

      reader.readAsDataURL(imagePortada);

      //#endregion

      fdImage.append('image', imagePortada, imagePortada.name);

      this.fileServ.upload(fdImage).subscribe(
        (res: any) => {

          this.portadaImage.thumbail = tempCoverThumbail;
          this.portadaImage.url = res.files[0].url;
          this.formPublication.get('cover').setValue(this.portadaImage.url);

          this.spinnerService.hide();

        }, (err: Error) => {

          // TODO: Manejar error por si se cae S3
          console.error(err);
          this.spinnerService.hide();

        }
      );

    };

  }

  //#region Realizar borrador

  public guardarBorrador(): void {
    if (!this.formPublication.valid) {
      this.swalFormDataInvalid.fire();
      return;
    }

    console.log(this.formPublication.controls);

    console.log('TEST > Guardar en borrador');

    this.swalFormDataValid.fire();
  }

  //#endregion

  //#region Realizar publicación
  public  submitPublicationForm(): void {


    this.formPublication.get('tags').setValue(this.tagsList);

    // Reducir descripción los espacios vacios que pueda tener al final
    const description = this.formPublication.get('synopsis');
    this.formPublication.get('synopsis').setValue(description.value.trim());

    console.log('TEST > Submit Publication > ', this.formPublication.value );

    if ( this.formPublication.invalid ) {

      this.swalFormDataInvalid.fire();

      return Object.values( this.formPublication.controls ).forEach( control => {

        // Si es un objeto
        if ( control instanceof FormGroup ) {

          Object.values( control.controls ).forEach( subControl => subControl.markAsTouched() );

        } else {

          control.markAsTouched(); // Marcar todos como tocadas

        }

      } );

    } else {

      this.newPublication();

    }

  }

  private newPublication(): void {

    this.spinnerService.show();
    const arrayPageObervables: Observable<any>[] = [];

    this.pagesList.controls.forEach( (pageForm: FormGroup) => {

      const page = pageForm.get('image').value;
      arrayPageObervables.push( this.fileServ.upload( page )  );

    });

    forkJoin( arrayPageObervables ).subscribe( ( dataPage ) => {

      const pagesUrl: IUpdatePagesCommand = this.getUrlFromFileService(dataPage);
      this.uploadPublicationPages(pagesUrl);

    }, (error: Error) => {

      console.error(error);
      this.spinnerService.hide();

    } );



  }

  private uploadPublicationPages( pagesUrlToUpload: IUpdatePagesCommand ): void {

    const createSketch: ICreateCommand = {

      name: this.formPublication.get('name').value,
      cover: this.formPublication.get('cover').value,
      synopsis: this.formPublication.get('synopsis').value,
      category_id: this.formPublication.get('category_id').value,
      tags: this.formPublication.get('tags').value,

    };

    this.publicationService.create( createSketch ).subscribe(

      // Se crea primero el borrador - TODO: Llamar a la función de creador borrador con una condición si luego publicar
      (resSketch: any) => {

        const idSketch = resSketch.id;

        console.log('ID PUB: ', resSketch);

        // Subir las paginas
        this.publicationService.updatePages( resSketch.id, pagesUrlToUpload  ).subscribe(
          (resPagesUpload: any) => {

            this.uploadPublicationFinish(resSketch.id);

          },
          (error: Error) => {

            console.error(error);
            this.spinnerService.hide();

          }
        );

      },
      (error: Error) => {

        console.error(error);
        this.spinnerService.hide();

      }

    );

  }

  private uploadPublicationFinish( idPublication: string ): void {

    // Realizar la publicación en sí con todos los datos necesarios.
    this.publicationService.publish( idPublication ).subscribe(
      (resPublish: any) => {

        this.swalFormDataValid.fire();
        this.spinnerService.hide();

      },
      (error: any) => {

        console.error(error);
        this.spinnerService.hide();

      }
    );

  }

  //#endregion


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

      this.tagsList.push(value);

    }

    // Reset the input value
    if (input) {
      input.value = '';
    }
  }

  public removeTag( tag: string ): void {

    const index = this.tagsList.indexOf(tag);

    if (index >= 0) {
      this.tagsList.splice(index, 1);
    }

  }

  // #endregion

  //#region Pages

  public addPage(): void {

      const newPage = this.newPage();

      // Crear elemento input de tipo 'file' para poder manejarlo desde el botón que lo llama
      const inputFileElement = document.createElement('input');
      inputFileElement.type = 'file'; // Nota:  Solo uno a la vez, para varios: inputFileElement.multiple = multiple
      inputFileElement.accept = '.png, .jpg, .jpeg';
      inputFileElement.click();

      // Definir la función del llamado al hacer click (cuando realiza un cambio)
      inputFileElement.onchange = ( event: any ) => {

        const fdImage: FormData = new FormData();
        const pageImage  = event.target.files[0];

        // #region Cargar para previsualizar en pantalla

        const reader = new FileReader();
        reader.onload = (eventReader: any ) => {

          newPage.get('thumbailImage').setValue(eventReader.target.result);

        };

        reader.readAsDataURL(pageImage);

        //#endregion

        fdImage.append('image', pageImage, pageImage.name);
        newPage.get('image').setValue(fdImage);

        this.pagesList.push( newPage );
        // console.log('TEST > ', newPage );
        // console.log('TEST > ', fdImage.getAll('image') );

      };


  }

  public removePage( index: number ): void {

    this.pagesList.removeAt( index );

    // console.log('TEST> ', this.pagesList.value[i]);

    const listLength = this.pagesTotal;
    for ( let i = index; i < listLength; i++ ) {

      this.pagesList.value[i].number = i + 1;

    }

  }

  public newPage(): FormGroup {
    return this.fb.group({

      number: this.pagesList.length + 1,
      image: [new FileReader(), Validators.required  ],
      thumbailImage: '',
      url: ''

    });
  }

  public getUrlFromFileService( dataFilePageUploaded: any[] ): IUpdatePagesCommand {

    const auxStringUrlPage: IUpdatePagesCommand = {
      pages: [] = []
    };

    dataFilePageUploaded.forEach( pageUploaded => {

      auxStringUrlPage.pages.push( {
        images: [pageUploaded.files[0].url]
      });

    });

    return auxStringUrlPage;

  }

  //#endregion

  // #region Getters

  get nombreNovalido(): boolean {
    return ( this.formPublication.get('name').invalid && this.formPublication.get('name').touched );
  }

  get synopsisNovalido(): boolean {
    return ( this.formPublication.get('synopsis').invalid && this.formPublication.get('synopsis').touched );
  }

  get synopsisLenght(): number {
    return this.formPublication.get('synopsis').value.length;
  }

  get pagesList(): FormArray  {
    return this.formPublication.get('pagesList') as FormArray;
  }

  get pagesTotal(): number {
    return this.pagesList.length;
  }

  get collectionArrayCheck(): FormArray {
    return this.formPublication.get('collectionArray') as FormArray;
  }

  //#endregion

}
