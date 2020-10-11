import { Component, OnInit, ViewChild } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { ActivatedRoute, Router } from '@angular/router';
import { NgxSpinnerService } from 'ngx-spinner';
import {COMMA, ENTER} from '@angular/cdk/keycodes';
import { AuthService } from 'src/app/domain/services/auth.service';
import { FileService } from 'src/app/domain/services/file.service';
import { IDropdownItem } from 'src/app/models/dropdown-item.interface';
import { DropdownDataObrasService } from 'src/app/services/dropdown-data-obras.service';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';
import { MatChipInputEvent } from '@angular/material/chips';
import { SwalComponent } from '@sweetalert2/ngx-sweetalert2';
import { ICreateCommand, CollectionService, ICreateResponse } from '../../../domain/services/collection.service';
import { ICollection } from 'src/app/domain/models';
import Swal from 'sweetalert2';
import { faTrashAlt } from '@fortawesome/free-solid-svg-icons';

@Component({
  selector: 'app-collection-new-edit',
  templateUrl: './collection-new-edit.component.html',
  styleUrls: ['./collection-new-edit.component.scss']
})
export class CollectionNewEditComponent implements OnInit {
  @ViewChild('formDataInvalid') private swalFormDataInvalid: SwalComponent;
  @ViewChild('collectionValid') private swalCollectionValid: SwalComponent;
  @ViewChild('formEditCollectionValid') private swalFormEditCollectionValid: SwalComponent;

  // FontAwesome Icon
  public faDelete = faTrashAlt;

  // Usados para forms
  public formCollection: FormGroup;
  public categoryList: IDropdownItem[];

  public portadaImage = {
    thumbail: null,
    url: null
  };

  public tagsList: string[] = [];

  // Otros
  public isToEdit: boolean;
  private collectionToEditId: string;

  public chipTagsKeysCodes: number[] = [ENTER, COMMA]; // Usado para los tags

  constructor(
    private router: Router,
    private authService: AuthService,
    private spinnerService: NgxSpinnerService,
    private activateRoute: ActivatedRoute,
    private fileServ: FileService,
    private fb: FormBuilder,
    private collectionService: CollectionService,
    private dropdownDataObrasService: DropdownDataObrasService,
    private sweetAlertGenericService: SweetAlertGenericMessageService,
  ) {
  }

  ngOnInit(): void {
    this.authService.authStart();
    this.buildForms();
    this.setSubscriptionData();
  }


  private buildForms(): void {


    this.formCollection = this.fb.group({

      name: ['', [ Validators.required, Validators.minLength(5) ] ],
      cover: ['', Validators.required ],
      synopsis: [ '', [ Validators.required, Validators.minLength(5),  Validators.maxLength(512) ] ],
      category_id: [ '', Validators.required ],
      tags: [ null ],
    });

  }

  private setFormCollectionByObject( collectionObject: ICollection ): void {

    this.formCollection.reset({

      name: collectionObject.name,
      cover: collectionObject.cover,
      synopsis: collectionObject.synopsis,
      category_id: collectionObject.category_id,

    });

    // Manejo de portada
    this.portadaImage.thumbail = collectionObject.cover;
    this.portadaImage.url = collectionObject.cover;

    // Asignar tags
    collectionObject.tags.forEach( (tag: string) => {

      this.tagsList.push(tag);

    });

    this.formCollection.get('tags').setValue(this.tagsList);

  }

  private setSubscriptionData(): void {

    this.spinnerService.show();

    this.dropdownDataObrasService.getAllCategoryDropdown().subscribe(
      ( resCategory: any) => {

        this.categoryList = resCategory;

        // Obtener ID si es un edit
        this.activateRoute.params.subscribe(
          (params: any) => {

            this.collectionToEditId = params.id;

            // Si no existe el id, es una nueva publicación, sino se busca con el ID la publicación
            if ( this.collectionToEditId === undefined ) {

              this.isToEdit = false;

            } else {

              this.isToEdit = true;
              this.getCollectionToEdit(this.collectionToEditId);

            }

            this.spinnerService.hide();

          }
        );

      },
      (err: Error) =>  {

        console.log( err );
        this.spinnerService.hide();

      }
    );

  }

  private getCollectionToEdit( collectionId: string ): void {

    this.spinnerService.show();

    this.collectionService.getById( collectionId ).subscribe(
      ( resCollection: ICollection ) => {

        this.spinnerService.hide();
        this.setFormCollectionByObject( resCollection );

      },
      ( err: Error ) => {
        this.spinnerService.hide();
        console.error(err);
      }
    );

  }

  public backToDeskboard(): void {
    this.router.navigate(['/deskboard'] );
  }

  public submitCollectionForm(): void {

    this.formCollection.get('tags').setValue(this.tagsList);

    // Reducir descripción los espacios vacios que pueda tener al final
    const description = this.formCollection.get('synopsis');
    this.formCollection.get('synopsis').setValue(description.value.trim());

    // console.log('TEST > Submit Publication > ', this.formCollection.value );
    if ( this.formCollection.invalid ) {

      if ( this.formCollection.get('cover').invalid ) {

        this.sweetAlertGenericService.showAlertError('Se requiere una portada para crear la colección');

      } else {

        this.swalFormDataInvalid.fire();

      }

      return Object.values( this.formCollection.controls ).forEach( control => {

        // Si es un objeto
        if ( control instanceof FormGroup ) {

          Object.values( control.controls ).forEach( subControl => subControl.markAsTouched() );

        } else {

          control.markAsTouched(); // Marcar todos como tocadas

        }

      } );

    } else {

      const collectionObject: ICreateCommand = {

        name: this.formCollection.get('name').value,
        cover: this.formCollection.get('cover').value,
        synopsis: this.formCollection.get('synopsis').value,
        category_id: this.formCollection.get('category_id').value,
        tags: this.formCollection.get('tags').value,

      };

      this.isToEdit ? this.submitCollectionEdit(collectionObject) : this.submitCollectionNew(collectionObject);

    }

  }

  private submitCollectionEdit( editCollectionObj: ICreateCommand ): void {

    this.spinnerService.show();
    this.collectionService.update( this.collectionToEditId, editCollectionObj ).subscribe(

      ( resCollectionEdit: any ) => {

        this.swalFormEditCollectionValid.fire();
        this.spinnerService.hide();

      },
      (err: Error ) => {

        console.error(err);
        this.sweetAlertGenericService.showAlertError('No se ha podido editar la colección', 'Error Servidor');
        this.spinnerService.hide();

      }

    );

  }

  private submitCollectionNew( createCollectionObj: ICreateCommand ): void {

    this.spinnerService.show();
    this.collectionService.create( createCollectionObj ).subscribe(

      ( resCollectionCreate: ICreateResponse ) => {

        this.swalCollectionValid.fire();
        this.spinnerService.hide();

      },
      (err: Error ) => {

        console.error(err);
        this.sweetAlertGenericService.showAlertError('No se ha podido crear la colección', 'Error Servidor');
        this.spinnerService.hide();

      }

    );

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
          this.formCollection.get('cover').setValue(this.portadaImage.url);

          this.spinnerService.hide();

        }, (err: Error) => {

          // TODO: Manejar error por si se cae S3
          console.error(err);
          this.spinnerService.hide();

        }
      );

    };

  }

  //#region Tags

  public removeTag( tag: string ): void {

    const index = this.tagsList.indexOf(tag);

    if (index >= 0) {
      this.tagsList.splice(index, 1);
    }

  }

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

  //#endregion


  public onCollectionDelete(): void {

    Swal.fire({
      title: 'Eliminar colección',
      text: '¿Estas seguro ',
      icon: 'warning',
      showCancelButton: true,
      focusCancel: true,
      confirmButtonColor: '#FC4850',
      cancelButtonColor: '#3085d6',
      confirmButtonText: 'Sí, eliminar',
      cancelButtonText: 'No'
    }).then((result) => {
      if (result.isConfirmed) {

        this.deleteCollectionConfirm();

      }
    });

  }

  private deleteCollectionConfirm(): void {

    this.spinnerService.show();

    this.collectionService.delete( this.collectionToEditId ).subscribe(
      (res: any) => {

        Swal.fire(
          'Eliminado con éxito',
          `${ this.formCollection.get('name').value } se ha eliminado.`,
          'success'
        ).then((result) => {
          if (result.isConfirmed) {

            this.backToDeskboard();

          }
        });

        this.spinnerService.hide();

      },
      (err: Error) => {

        this.sweetAlertGenericService.showAlertError('Ha ocurrido un problema al eliminar la colección', 'Error Servidor');
        console.error(err);

        this.spinnerService.hide();

      }
    );

  }

  //#region Getters

  get nombreNovalido(): boolean {
    return ( this.formCollection.get('name').invalid && this.formCollection.get('name').touched );
  }

  get categoryNoValido(): boolean {
    return ( this.formCollection.get('category_id').invalid && this.formCollection.get('category_id').touched );
  }

  get categoryFormValueId(): string {

    return this.formCollection.get('category_id').value;

  }

  get categoryValueName(): string {

    return this.categoryList.filter( element => {

      return (element.valueId === this.categoryFormValueId);

    } )[0].name;
  }

  get synopsisNovalido(): boolean {
    return ( this.formCollection.get('synopsis').invalid && this.formCollection.get('synopsis').touched );
  }

  get synopsisLenght(): number {
    return this.formCollection.get('synopsis').value.length;
  }

  //#endregion

}
