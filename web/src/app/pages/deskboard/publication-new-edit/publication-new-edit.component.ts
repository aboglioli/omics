import { Component, OnInit, ViewChild } from '@angular/core';
import { Router, ActivatedRoute } from '@angular/router';
import { forkJoin, Observable } from 'rxjs';
import { FormGroup, FormBuilder, Validators, FormArray, FormControl } from '@angular/forms';
import { MatCheckbox } from '@angular/material/checkbox';
import {COMMA, ENTER} from '@angular/cdk/keycodes';

import { faPlusCircle, faTimesCircle, faBookOpen } from '@fortawesome/free-solid-svg-icons';

import { AuthService } from '../../../domain/services/auth.service';
import { FileService } from '../../../domain/services/file.service';
import { DropdownDataObrasService } from '../../../services/dropdown-data-obras.service';
import { NgxSpinnerService } from 'ngx-spinner';

import { IPublication, IPage } from '../../../domain/models/publication';
import { IDropdownItem } from '../../../models/dropdown-item.interface';
import { MatChipInputEvent } from '@angular/material/chips';
import { SwalComponent } from '@sweetalert2/ngx-sweetalert2';
import { PublicationService, IUpdatePagesCommand, IGetByIdResponse, IGetCollectionsResponse, IUpdateCommand, IReadResponse } from '../../../domain/services/publication.service';
import { ICreateCommand, CollectionService } from '../../../domain/services/collection.service';
import { SweetAlertGenericMessageService } from '../../../services/sweet-alert-generic-message.service';
import { ICollection } from '../../../domain/models/collection';

export interface IPageForm {
  number: number;
  image: string;
  thumbailImage: string;
  url: string;
}

export interface IPageAndNumber {
  number: number;
  url: string;
}

@Component({
  selector: 'app-publication-new-edit',
  templateUrl: './publication-new-edit.component.html',
  styleUrls: ['./publication-new-edit.component.scss']
})
export class PublicationNewEditComponent implements OnInit {

  @ViewChild('formDataInvalid') private swalFormDataInvalid: SwalComponent;
  @ViewChild('formDataValid') private swalFormDataValid: SwalComponent;
  @ViewChild('formSketchValid') private swalFormSketchValid: SwalComponent;
  @ViewChild('formEditPublishValid') private swalFormEditPublishValid: SwalComponent;

  // FontAwesome Icon
  public faPlus = faPlusCircle;
  public faCloseCircle = faTimesCircle;
  public faBoookOpen = faBookOpen;

  // Usados para Forms
  public formPublication: FormGroup;
  public collectionList: IDropdownItem[];
  public portadaImage = {
    thumbail: null,
    url: null
  };
  public categoryList: IDropdownItem[];
  public tagsList: string[] = [];


  // Otros
  private isToEdit: boolean;
  private isToSketch: boolean;
  private publicationToEditId: string;

  public chipTagsKeysCodes: number[] = [ENTER, COMMA]; // Usado para los tags

  constructor(
    private router: Router,
    private activateRoute: ActivatedRoute,
    private authService: AuthService,
    private fb: FormBuilder,
    private spinnerService: NgxSpinnerService,
    private dropdownDataObrasService: DropdownDataObrasService,
    private fileServ: FileService,
    private publicationService: PublicationService,
    private collectionService: CollectionService,
    private sweetAlertGenericService: SweetAlertGenericMessageService,
  ) { }

  ngOnInit(): void {

    this.isToSketch = false;

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

    const observableList =  [
        this.dropdownDataObrasService.getAllCollectionDropdownDataById( this.authService.getIdUser() ),
        this.dropdownDataObrasService.getAllCategoryDropdown()
    ];

    forkJoin( observableList).subscribe(([ dataCollection, dataCategory ]) => {

      this.collectionList = dataCollection;
      this.categoryList = dataCategory;

      this.activateRoute.params.subscribe(
        (params: any) => {

          this.publicationToEditId = params.id;

          // Si no existe el id, es una nueva publicación, sino se busca con el ID la publicación
          if ( this.publicationToEditId === undefined ) {

            this.isToEdit = false;

          } else {

            this.isToEdit = true;
            this.getPublicationToEdit(this.publicationToEditId);

          }

          this.spinnerService.hide();


        });

    });

  }

  private getPublicationToEdit( publicationId: string): void {

    this.spinnerService.show();

    this.publicationService.getById(publicationId).subscribe(
      (res: IGetByIdResponse) => {

        this.spinnerService.hide();
        const publicationEdit: IPublication = res.publication;
        this.publicationService.getCollections(publicationId).subscribe(
          (collectionRes: IGetCollectionsResponse) => {

            this.setFormPublicationByObject(publicationEdit, collectionRes.collections);

          },
          (err: Error) => {
            console.error(err);
            this.spinnerService.hide();
          }
        );

      },
      (err: Error ) => {

        console.error(err);
        this.spinnerService.hide();

      }
    );

  }

  private setFormPublicationByObject( publicationObject: IPublication, collectionList: ICollection[] ): void {

    // TODO: Esto habría que cambiarlo para que el tiempo que transcurra es cuando termine de renderizar las imagenes a mostrar
    this.spinnerService.show();
    setTimeout(() => {
      this.spinnerService.hide();
    }, 2000); // 5 segundos

    this.formPublication.reset({

      name: publicationObject.name,
      cover: publicationObject.cover,
      synopsis: publicationObject.synopsis,
      category_id: publicationObject.category_id,

    });

    // Asignar las colecciones
    if ( collectionList.length > 0 ) {

      collectionList.forEach( (collection: ICollection) => {

        this.collectionArrayCheck.push(
            new FormControl(collection.id)
        );

      });

    }

    // Manejo de portada
    this.portadaImage.thumbail = publicationObject.cover;
    this.portadaImage.url = publicationObject.cover;

    // Asignar tags
    publicationObject.tags.forEach( (tag: string) => {

      this.tagsList.push(tag);

    });

    this.formPublication.get('tags').setValue(this.tagsList);

    // Asignar pagesList
    publicationObject.pages.forEach( ( page: IPage ) =>  {

      const pageAux = this.newPage( page );
      this.pagesList.push( pageAux );

    });

  }
  // Generales
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

  public guardarBorrador(): void {

    this.isToSketch = true;
    this.submitPublicationForm();

  }


  //#region Realizar publicación
  public submitPublicationForm(): void {

    this.formPublication.get('tags').setValue(this.tagsList);

    // Reducir descripción los espacios vacios que pueda tener al final
    const description = this.formPublication.get('synopsis');
    this.formPublication.get('synopsis').setValue(description.value.trim());

    // console.log('TEST > Submit Publication > ', this.formPublication.value );

    if ( this.formPublication.invalid ) {

      if ( this.formPublication.get('cover').invalid ) {

        this.sweetAlertGenericService.showAlertError('Se requiere una portada para crear la publicación');

      } else {

        this.swalFormDataInvalid.fire();

      }

      return Object.values( this.formPublication.controls ).forEach( control => {

        // Si es un objeto
        if ( control instanceof FormGroup ) {

          Object.values( control.controls ).forEach( subControl => subControl.markAsTouched() );

        } else {

          control.markAsTouched(); // Marcar todos como tocadas

        }

      } );

    } else {

      if ( this.pagesTotal > 0 || this.isToSketch ) {

        this.spinnerService.show();

        if ( this.isToEdit ) {
          (this.pagesTotal > 0) ? this.editPublication() : this.editCollectionToPublication(this.publicationToEditId);
        } else {
          (this.pagesTotal > 0) ? this.newPublication() : this.uploadPublicationNewPages();
        }


      } else {

        this.sweetAlertGenericService.showAlertError( 'No hay páginas cargadas a la colección.' );

      }


    }

  }

  private newPublication(): void {

    const arrayPageObervables: Observable<any>[] = [];
    let pagesUrl: IUpdatePagesCommand = {
      pages: [] = []
    };

    this.pagesList.controls.forEach( (pageForm: FormGroup) => {

      const page = pageForm.get('image').value;

      arrayPageObervables.push( this.fileServ.upload( page )  );

    });

    forkJoin( arrayPageObervables ).subscribe( ( dataPage ) => {

      // console.log( 'TEST DATAPAGE > ', dataPage  );
      pagesUrl = this.getUrlFromFileService(dataPage);
      this.uploadPublicationNewPages(pagesUrl);

    }, (error: Error) => {

      console.error(error);
      this.spinnerService.hide();

    } );



  }

  private editPublication(): void {

    // #region Crear un arreglo de páginas con el total que existen y asignarles un número
    const pagesOrderByNumberList: IPageAndNumber[] = [];

    for ( let i = 0; i < this.pagesTotal; i++ ) {

      pagesOrderByNumberList.push({
        number: i + 1,
        url: null
      });

    }

    // #endregion

    // #region Obtener primero todos los que no se modificaron (se asignan nomas con las del número que corresponde)
    this.pagesList.controls.forEach( (pageForm: FormGroup) => {

      const isNewPage = pageForm.get('image').value; // Si es nulo, es una página no actualizada...sino es nueva.

      if (  !isNewPage ) {

        const oldPage = pageForm.value as IPageForm;

        pagesOrderByNumberList.map( page => {

          if ( page.number === oldPage.number ) {

            page.url = oldPage.url;

          }

        });

      }

    });

    //#endregion

    //#region  Obtener todas las páginas nuevas
    const arrayNewPageObervables: Observable<any>[] = [];
    const pagesUrl: IUpdatePagesCommand = {
      pages: [] = []
    };

    this.pagesList.controls.forEach( (pageForm: FormGroup) => {

      const page = pageForm.get('image').value;

      // No es nulo el campo 'image' => es una página cargada nueva
      if ( page ) {
        arrayNewPageObervables.push( this.fileServ.upload( page )  );
      }

    });


    // Si no se ha cargado una nueva imagen, ir directo a la carga de las págins sino esperar a que se asignen la url necesaria
    if ( arrayNewPageObervables.length === 0 ) {

      // << Sin paginas nuevas >>
      pagesOrderByNumberList.forEach( (page: IPageAndNumber) => {

        pagesUrl.pages.push({
          images: [page.url]
        });

      });

      this.uploadPublicationEditPages(pagesUrl);

    } else {

      // << Con páginas nuevas >>
      let auxPagesNewUrl: IUpdatePagesCommand = {
        pages: [] = []
      };

      // #region Obtener los url de las nuevas páginas
      forkJoin( arrayNewPageObervables ).subscribe( ( dataPage ) => {

        auxPagesNewUrl = this.getUrlFromFileService(dataPage);

        // #region Asignar los url obtenidos, en orden, en los espacios sin url creados en el arreglo anteriormente

        auxPagesNewUrl.pages.forEach( (pageNew: any) => {

          pagesOrderByNumberList.find( page => !page.url ).url = pageNew.images[0];

        });


        // #endregion

        // --> Enviar para colocarlos en una publicación ya existente
        pagesOrderByNumberList.forEach( (page: IPageAndNumber) => {

          pagesUrl.pages.push({
            images: [page.url]
          });

        });

        this.uploadPublicationEditPages(pagesUrl);

      }, (error: Error) => {

        console.error(error);
        this.spinnerService.hide();

      } );

    }

    //#endregion

  }

  private uploadPublicationNewPages( pagesUrlToUpload?: IUpdatePagesCommand ): void {

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

        // Si solo se quería generar el nuevo id, se saltea el paso de asignar las páginas

        if ( !pagesUrlToUpload ) {

          this.assignCollectionToPublication(idSketch);

        } else {

          // Subir las paginas
          this.publicationService.updatePages( idSketch, pagesUrlToUpload  ).subscribe(
            (resPagesUpload: any) => {

              this.assignCollectionToPublication(idSketch);

            },
            (error: Error) => {

              console.error(error);
              this.spinnerService.hide();

            }
          );
        }

      },
      (error: Error) => {

        console.error(error);
        this.spinnerService.hide();

      }

    );

  }

  private uploadPublicationEditPages( pagesUrlToUpload: IUpdatePagesCommand ): void {

    // Subir las paginas
    this.publicationService.updatePages( this.publicationToEditId, pagesUrlToUpload  ).subscribe(
      (resPagesUpload: any) => {

        this.editCollectionToPublication(this.publicationToEditId);

      },
      (error: Error) => {

        console.error(error);
        this.spinnerService.hide();

      }
    );


  }


  private assignCollectionToPublication( idPublication: string ): void {

    if ( this.collectionArrayCheck.controls.length === 0 ) {

      ( this.isToEdit || this.isToSketch ) ?
        this.editPublicationFinish(idPublication) : this.newPublicationFinish(idPublication) ;

    } else {

      // Primero se crea una lista con todas los observables a usar para añadir la collección a la publicación creada
      const collectionSubscriptionsList: any[] = [];

      this.collectionArrayCheck.controls.forEach( (collection: FormControl) => {

        collectionSubscriptionsList.push( this.collectionService.addPublication( collection.value , idPublication ) );

      } );

      // Se realiza la subscripción de todas las colección al Id de publicación y si está correcto, se publica finalmente
      forkJoin(  collectionSubscriptionsList ).subscribe(
        (data: any) => {

          ( this.isToEdit || this.isToSketch ) ?
            this.editPublicationFinish(idPublication) : this.newPublicationFinish(idPublication) ;

        },
        (error: Error) => {

          console.error(error);
          this.spinnerService.hide();

        }
      );

    }


  }

  private editCollectionToPublication( idPubcalition: string ): void {

    /*
      Se consiguen todas las colecciones que tiene la publicacíón a editar, si no tiene se
    va a la función que asigna las mismas. Caso que tenga alguna se quitan las mismas de la publicación
    antes de ir a asignarselas las que se seleccionaron al editar.

      TODO: Seguramente hay una forma mejor de hacer esto, es lo que se me ocurrió.
    */
    let collectionToRemove: ICollection[] = [];

    this.publicationService.getCollections( idPubcalition ).subscribe(
      ( resCollection: IGetCollectionsResponse ) => {

        collectionToRemove = resCollection.collections;

        if ( collectionToRemove.length === 0 ) {
          this.assignCollectionToPublication(idPubcalition);
        } else {

          const collectionRemoveSubscriptionList: any[] = [];
          collectionToRemove.forEach( (collection: ICollection) => {

            // console.log( 'TEST >>', collection.id );
            collectionRemoveSubscriptionList.push( this.collectionService.removePublication( collection.id, idPubcalition ) );

          });

          forkJoin( collectionRemoveSubscriptionList ).subscribe(

            (res: any) => {
              this.assignCollectionToPublication(idPubcalition);
            },
            (err: Error) => {
              console.error(err);
              this.spinnerService.hide();
            }

          );

        }

      }
    );

  }

  private newPublicationFinish( idPublication: string ): void {

    // Realizar la publicación en sí con todos los datos necesarios.
    this.publicationService.publish( idPublication ).subscribe(
      (resPublish: any) => {

        // Decidir es por edición (en ese caso: nueva publicaicón o borrador) o publicación totalmente nueva
        if ( this.isToEdit ) {

          this.swalFormEditPublishValid.fire();

        } else {

          this.swalFormDataValid.fire();

        }

        this.spinnerService.hide();

      },
      (error: any) => {

        console.error(error);
        this.spinnerService.hide();

      }
    );

  }

  private editPublicationFinish( idPublication: string ): void {


    const publicationUpdateCMD: IUpdateCommand = {

      name: this.formPublication.get('name').value,
      synopsis: this.formPublication.get('synopsis').value,
      category_id: this.formPublication.get('category_id').value,
      tags: this.formPublication.get('tags').value,
      cover: this.formPublication.get('cover').value,

    };

    // Editar la publicación en sí con todos los datos necesarios.
    this.publicationService.update( idPublication, publicationUpdateCMD ).subscribe(
      (resEditPublish: any) => {

        if ( this.isToSketch ) {

          this.swalFormSketchValid.fire();
          this.isToSketch = false; // Por si luego desea publicar (puede volver a guardarlo como borrador)

          this.spinnerService.hide();

        } else {

          this.newPublicationFinish(idPublication);

        }

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

      };


  }

  public removePage( index: number ): void {

    this.pagesList.removeAt( index );


    const listLength = this.pagesTotal;
    for ( let i = index; i < listLength; i++ ) {

      this.pagesList.value[i].number = i + 1;

    }

  }

  private newPage( pageObject?: IPage ): FormGroup {

    if (  pageObject ) {

      const urlPage = pageObject.images[0].url;

      return this.fb.group({

        number: (pageObject.number) + 1,
        image: null, // TODO: Corroborar si esto hace falta
        thumbailImage: urlPage,
        url: urlPage

      });

    }
    else {

      return this.fb.group({

        number: this.pagesList.length + 1,
        image: [new FileReader(), Validators.required  ],
        thumbailImage: '',
        url: ''

      });
    }

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

  get categoryNoValido(): boolean {
    return ( this.formPublication.get('category_id').invalid && this.formPublication.get('category_id').touched );
  }

  get categoryFormValueId(): string {

    return this.formPublication.get('category_id').value;

  }

  get categoryValueName(): string {

    return this.categoryList.filter( element => {

      return (element.valueId === this.categoryFormValueId);

    } )[0].name;
  }

  //#endregion

}
