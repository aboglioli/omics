import { BrowserModule } from '@angular/platform-browser';
import { HTTP_INTERCEPTORS, HttpClientModule } from '@angular/common/http';
import { NgModule } from '@angular/core';

import { AuthorService } from './services/author.service';
import { AuthService, AuthInterceptor } from './services/auth.service';
import { CategoryService } from './services/category.service';
import { CollectionService } from './services/collection.service';
import { ConfigService } from './services/config.service';
import { ContractService } from './services/contract.service';
import { FileService } from './services/file.service';
import { IdentityService } from './services/identity.service';
import { NotificationService } from './services/notification.service';
import { PlanService } from './services/plan.service';
import { PublicationService } from './services/publication.service';
import { ReaderService } from './services/reader.service';
import { SubscriptionService } from './services/subscription.service';

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
    FileService,
    IdentityService,
    NotificationService,
    PublicationService,
    ReaderService,
    PlanService,
    ContractService,
    SubscriptionService,

    // Auth interceptor
    {
      provide: HTTP_INTERCEPTORS,
      useClass: AuthInterceptor,
      multi: true,
    },
  ],
})
export class DomainModule {}
