import { Component, OnInit } from '@angular/core';
import { NgxSpinnerService } from 'ngx-spinner';
import { forkJoin } from 'rxjs';
import { IGetAllResponse } from 'src/app/domain/services/category.service';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';
import { ICategory } from '../../../domain/models/category';
import { CategoryService, ICreateCommand } from '../../../domain/services/category.service';
import { faEdit, faPlusCircle, faSave, faTimesCircle } from '@fortawesome/free-solid-svg-icons';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { AuthService } from "../../../domain/services/auth.service";
import { IUser, can } from "../../../domain/models/user";

@Component({
  selector: 'app-admin-categories',
  templateUrl: './admin-categories.component.html',
  styleUrls: ['./admin-categories.component.scss']
})
export class AdminCategoriesComponent implements OnInit {

  // Font Awseome icons
  public faEdit = faEdit;
  public faAdd = faPlusCircle;
  public faCancel = faTimesCircle;

  public categoryList: ICategory[];
  public isEditCategory: boolean;
  private editIndexCategory: number;

  public formCategory: FormGroup;

  public user: IUser;
  public can = can;

  constructor(
    private categoryService: CategoryService,
    private spinnerService: NgxSpinnerService,
    private sweetAlertGenericService: SweetAlertGenericMessageService,
    private fb: FormBuilder,
    private authService: AuthService,
  ) { }

  ngOnInit(): void {

    this.isEditCategory = false;
    this.formBuild();
    this.getCategories();

    this.authService.getUser().subscribe((user) => {
      this.user = user;
    });

  }

  private formBuild(): void {

    this.formCategory = this.fb.group({
      name: [ '', [ Validators.required, Validators.minLength(2) ] ],
      id: [''],
      created_at: [ new Date() ],
      updated_at: [ new Date()]
    });

  }

  public resetForm(): void {
    this.formCategory.reset();
    this.isEditCategory = false;
    this.editIndexCategory = -1;
  }

  private getCategories(): void {

    this.spinnerService.show();
    this.categoryService.getAll().subscribe(
      (categoryRes: IGetAllResponse) => {

        this.spinnerService.hide();
        this.categoryList = categoryRes.categories;

        // console.log('TEST > ', this.categoryList);

      },
      (err: Error) => {

        this.spinnerService.hide();
        console.error(err);

      }
    );

  }

  public removeCategory( category: ICategory, index: number ): void {

    let canDeleteCategory = true;

    // Verificar primero que no tenga alguna publicación asignada
    this.spinnerService.show();
    forkJoin([
      this.categoryService.getPublications(category.id),
      this.categoryService.getCollections(category.id),
    ]).subscribe(([pubRes, collRes]) => {

      this.spinnerService.hide();

      if (pubRes.matching_criteria > 0 || collRes.matching_criteria > 0) {
        canDeleteCategory = false;
      }

      if ( canDeleteCategory ) {

        this.categoryService.delete(category.id).subscribe(() => {});
        this.categoryList.splice(index, 1);

        this.sweetAlertGenericService.showAlertSuccess(`La categoría ${ category.name } ha sido eliminado`, 'Eliminado');

      } else {

        this.sweetAlertGenericService.showAlertError(
          'Las categorías con publicaciones y/o colecciones asignadas, no pueden eliminarse. Contactarse con el encargado de la base de datos.',
          `No puede eliminarse ${ category.name }`
        );

      }

    });

  }

  public editCategory( category: ICategory, index: number ): void {

    this.isEditCategory = true;
    this.editIndexCategory = index;
    this.setEditFormByObject(category);

  }

  private setEditFormByObject( category: ICategory): void {

    this.formCategory.reset({

      name: category.name,
      id: category.id,
      created_at: category.created_at

    });

  }

  public onSubmitForm(): void {

    const categoryObject: ICategory = {

      id: this.formCategory.get('id').value,
      name: this.formCategory.get('name').value.trim(),
      created_at: this.formCategory.get('created_at').value,
      updated_at: this.formCategory.get('updated_at').value

    };

    this.spinnerService.show();
    if ( this.isEditCategory ) {

      // Editar
      this.categoryService.update( categoryObject.id, {name: categoryObject.name} ).subscribe(
        (res) => {

          this.spinnerService.hide();

          console.log(this.editIndexCategory);
          this.categoryList[this.editIndexCategory].name = categoryObject.name;
          this.sweetAlertGenericService.showAlertSuccess(`La categoría ${ categoryObject.name } ha sido modificada`, 'Modificado');
          this.resetForm();

        },
        (err: Error) => {

          this.spinnerService.hide();
          console.error(err);

        }
      );

    } else {

      // Crear
      this.categoryService.create( { name: categoryObject.name } ).subscribe(

        (res) => {
          this.spinnerService.hide();

          console.log(res);
          categoryObject.id = res.id;
          this.categoryList.push( categoryObject );

          this.sweetAlertGenericService.showAlertSuccess(`La categoría ${ categoryObject.name } ha creada`, 'Creado');

        },
        (err: any) => {

          this.spinnerService.hide();

          if (  err.error.code === 'already_exists' ) {

            this.sweetAlertGenericService.showAlertError('Este nombre de categoría ya ha sido utilizado', 'Nombre duplicado');

          } else {

            console.error(err);

          }

        }

      );

    }

  }

  // Getters
  get nombreNovalido(): boolean {
    return ( this.formCategory.get('name').invalid && this.formCategory.get('name').touched );
  }

}
