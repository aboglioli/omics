import { BrowserModule } from '@angular/platform-browser';
import { HTTP_INTERCEPTORS, HttpClientModule } from '@angular/common/http';
import { NgModule } from '@angular/core';

import { AuthorService } from './services/author';
import { AuthService, AuthInterceptor } from './services/auth';
import { CategoryService } from './services/category';
import { CollectionService } from './services/collection';
import { ConfigService } from './services/config';
import { IdentityService } from './services/identity';
import { PublicationService } from './services/publication';

@NgModule({
  imports: [
    BrowserModule,
    HttpClientModule,
  ],
  providers: [
    AuthorService,
    AuthorService,
    CategoryService,
    CollectionService,
    ConfigService,
    IdentityService,
    PublicationService,

    // Auth interceptor
    {
      provide: HTTP_INTERCEPTORS,
      useClass: AuthInterceptor,
      multi: true,
    },
  ],
})
export class DomainModule {}
