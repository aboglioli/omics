import { Component, OnInit } from '@angular/core';
import { NgxSpinnerService } from 'ngx-spinner';
import { IGetAllResponse } from 'src/app/domain/services/category.service';
import { ICategory } from '../../../domain/models/category';
import { CategoryService } from '../../../domain/services/category.service';

@Component({
  selector: 'app-admin-categories',
  templateUrl: './admin-categories.component.html',
  styleUrls: ['./admin-categories.component.scss']
})
export class AdminCategoriesComponent implements OnInit {

  public categoryList: ICategory[];

  constructor(
    private categoryService: CategoryService,
    private spinnerService: NgxSpinnerService,
  ) { }

  ngOnInit(): void {

    this.getCategories();

  }

  private getCategories(): void {

    this.spinnerService.show();
    this.categoryService.getAll().subscribe(
      (categoryRes: IGetAllResponse) => {

        this.spinnerService.hide();
        this.categoryList = categoryRes.categories;

        console.log('TEST > ', this.categoryList)

      },
      (err: Error) => {

        this.spinnerService.hide();
        console.error(err);

      }
    )

  }

}
