import { BrowserModule } from '@angular/platform-browser';
import { HTTP_INTERCEPTORS, HttpClientModule } from '@angular/common/http';
import { NgModule } from '@angular/core';

import { AuthorService } from './services/author.service';
import { AuthService, AuthInterceptor } from './services/auth.service';
import { CategoryService } from './services/category.service';
import { CollectionService } from './services/collection.service';
import { ConfigService } from './services/config.service';
import { IdentityService } from './services/identity.service';
import { PublicationService } from './services/publication.service';
import { FileService } from './services/file.service';

@NgModule({
  imports: [
    BrowserModule,
    HttpClientModule,
  ],
  providers: [
    AuthorService,
    AuthService,
    CategoryService,
    CollectionService,
    ConfigService,
    IdentityService,
    PublicationService,
    FileService,

    // Auth interceptor
    {
      provide: HTTP_INTERCEPTORS,
      useClass: AuthInterceptor,
      multi: true,
    },
  ],
})
export class DomainModule {}
