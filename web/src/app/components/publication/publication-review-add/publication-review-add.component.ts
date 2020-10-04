import { Component, OnInit, Inject, ViewChild } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material/dialog';
import { faTimesCircle } from '@fortawesome/free-solid-svg-icons';
import { SwalComponent } from '@sweetalert2/ngx-sweetalert2';
import { NgxSpinnerService } from 'ngx-spinner';
import { PublicationService, IAddReviewCommand } from '../../../domain/services/publication.service';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';
import { IReview } from '../../../domain/models/review';

export interface DialogData {
  rating: number;
  idPublication: string;
  review: IReview;
}

@Component({
  selector: 'app-publication-review-add',
  templateUrl: './publication-review-add.component.html',
  styleUrls: ['./publication-review-add.component.scss']
})
export class PublicationReviewAddComponent implements OnInit {

  @ViewChild('formDataInvalid') private swalFormDataInvalid: SwalComponent;
  @ViewChild('formDataValid') private swalFormDataValid: SwalComponent;

  // Font Awseome icons
  public faClose = faTimesCircle;

  public ratingNumber = 1;
  public formReview: FormGroup;

  private publicationId: string;

  constructor(
    @Inject(MAT_DIALOG_DATA) public data: DialogData,
    public dialogRef: MatDialogRef<PublicationReviewAddComponent>,
    private fb: FormBuilder,
    private publicationService: PublicationService,
    private spinnerService: NgxSpinnerService,
    private sweetAlertGenericService: SweetAlertGenericMessageService
  ) {
    dialogRef.disableClose = true;
  }

  ngOnInit(): void {

    this.ratingNumber = this.data.rating;
    this.publicationId = this.data.idPublication;

    const auxComment = (this.data.review) ? this.data.review.comment : '';
    this.formReview = this.fb.group({

      stars: [ this.ratingNumber, [ Validators.required, Validators.min(1) ] ],
      comment: [ auxComment, [ Validators.required, Validators.minLength(5), Validators.maxLength(256) ] ]

    });

  }

  public onClose( isReviewChanged: boolean ): void {

    this.dialogRef.close(isReviewChanged);

  }

  public onSendReview(): void {

    const auxComment = ( this.formReview.get('comment').value as string).trim();
    // this.formReview.get('comment').setValue()
    this.formReview.get('comment').setValue(auxComment);


    if ( this.formReview.invalid ) {

      this.swalFormDataInvalid.fire();

      return Object.values( this.formReview.controls ).forEach( control => {

        control.markAsTouched(); // Marcar todos como tocadas

      } );

    } else {

      let auxReviewObject: IAddReviewCommand;
      auxReviewObject = {
        stars: this.formReview.get('stars').value,
        comment: this.formReview.get('comment').value
      };

      this.spinnerService.show();

      // En caso que haya una review, primero se borra y luego se crea la nueva (no hay editar)
      if ( this.data.review ) {
        // this.publicationService.deleteReview
        this.publicationService.deleteReview(this.publicationId).subscribe(
          (res: any) => {
            this.addNewReview( auxReviewObject );
          },
          (err: Error) => {

            this.sweetAlertGenericService.showAlertError('No se ha agregado el análisis');
            this.spinnerService.hide();

          }
        )


      } else {

        this.addNewReview( auxReviewObject );

      }

    }

  }

  private addNewReview( reviewObject: IAddReviewCommand ): void {

    this.publicationService.addReview( this.publicationId, reviewObject ).subscribe(

      (res: any) => {

        this.ratingNumber = reviewObject.stars;

        this.swalFormDataValid.fire();
        this.spinnerService.hide();

      },
      (err: Error) => {

        console.error(err);

        this.sweetAlertGenericService.showAlertError('No se ha agregado el análisis');
        this.spinnerService.hide();

      }

    );

  }

  get commentNovalido(): boolean {
    return ( this.formReview.get('comment').invalid && this.formReview.get('comment').touched );
  }

  get commentLenght(): number {

    const comment = this.formReview.get('comment').value;

    return ( comment ) ? comment.length : 0;

  }

}
