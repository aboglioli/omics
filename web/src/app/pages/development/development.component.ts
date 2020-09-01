import { FormBuilder, FormGroup } from '@angular/forms';
import { Component, OnInit } from '@angular/core';

import { IdentityService, IRegisterCommand, } from '../../domain/services/identity.service';
import { FileService, IUploadResponse } from '../../domain/services/file';

@Component({
  selector: 'app-development',
  templateUrl: './development.component.html',
})
export class DevelopmentComponent implements OnInit {
  public uploadForm: FormGroup;
  public uploadedImage: string;

  constructor(
    private formBuilder: FormBuilder,
    private identityServ: IdentityService,
    private fileServ: FileService,
  ) { }

  ngOnInit(): void {
    this.uploadForm = this.formBuilder.group({
      image: [''],
    });
  }

  onFileSelect(event): void {
    if (event.target.files.length > 0) {
      const file = event.target.files[0];
      this.uploadForm.get('image').setValue(file);
    }
  }

  onSubmit(): void {
    const formData = new FormData();
    formData.append('file', this.uploadForm.get('image').value, 'asd.jpg');

    this.fileServ.upload(formData).subscribe(
      (res: IUploadResponse) => {
        this.uploadedImage = res.files[0].url;
      },
      (error) => console.log(error),
    );
  }
}
