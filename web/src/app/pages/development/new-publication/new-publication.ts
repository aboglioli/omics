import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Component, OnInit } from '@angular/core';

import { PublicationService } from '../../../domain/services/publication';
import { CategoryService } from '../../../domain/services/category';
import { FileService } from '../../../domain/services/file';
import { ICategory } from '../../../domain/models';

@Component({
  selector: 'dev-new-publication',
  templateUrl: './new-publication.html',
  styleUrls: ['../general.scss'],
})
export class DevNewPublicationComponent implements OnInit {
  public form: FormGroup;
  public categories: ICategory[] = [];
  public coverUrl?: string;
  public error?: string;
  public success?: string;

  constructor(
    private formBuilder: FormBuilder,
    private publicationServ: PublicationService,
    private categoryServ: CategoryService,
    private fileServ: FileService,
  ) { }

  ngOnInit(): void {
    this.form = this.formBuilder.group({
      name: ['', [Validators.required, Validators.minLength(4), Validators.maxLength(256)]],
      synopsis: ['', [Validators.required, Validators.minLength(4), Validators.maxLength(1024)]],
      categoryId: ['', Validators.required],
      tags: [''],
    });

    this.categoryServ.getAll().subscribe(
      res => {
        this.categories = res.categories;
      },
      err => console.log(err),
    );
  }

  onCoverSelect(event): void {
    if (event.target.files.length > 0) {
      const file = event.target.files[0];

      const formData = new FormData();
      formData.append('file', file, file.name);

      this.fileServ.upload(formData).subscribe(
        res => {
          this.coverUrl = res.files[0].url;
        },
        err => console.log(err),
      );
    }
  }

  onSubmit(): void {

  }
}
