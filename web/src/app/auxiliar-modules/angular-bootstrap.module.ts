import { NgModule } from '@angular/core';

import {NgbAlertModule, NgbCarouselModule, NgbDropdownModule} from '@ng-bootstrap/ng-bootstrap';

@NgModule({
  declarations: [],
  imports: [
    NgbAlertModule,
    NgbDropdownModule,
    NgbCarouselModule
  ],
  exports: [

    NgbAlertModule,
    NgbDropdownModule,
    NgbCarouselModule

  ]
})
export class AngularBootstrapModule { }
