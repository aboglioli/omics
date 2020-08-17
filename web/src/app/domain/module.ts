import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { HttpClientModule } from '@angular/common/http';

import { ConfigService } from './services/config';
import { IdentityService } from './services/identity';
import { PublicationService } from './services/publication';
import { CollectionService } from './services/collection';

@NgModule({
  imports: [
    BrowserModule,
    HttpClientModule,
  ],
  providers: [
    ConfigService,
    IdentityService,
    PublicationService,
    CollectionService,
  ],
})
export class DomainModule {}
