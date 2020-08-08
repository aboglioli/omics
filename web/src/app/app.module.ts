import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { environment } from '../environments/environment';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';

import { HttpClientModule  } from '@angular/common/http';

import { ServiceWorkerModule } from '@angular/service-worker'; // PWA dependencia
import { ToastrModule } from 'ngx-toastr';
import { ChartsModule } from 'ng2-charts';

// Modulos auxiliares
import { MaterialModule } from './auxiliar-modules/material.module';
import { AngularBootstrapModule } from './auxiliar-modules//angular-bootstrap.module';


@NgModule({
  declarations: [
    AppComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    MaterialModule,
    AngularBootstrapModule,
    HttpClientModule,
    ServiceWorkerModule.register('ngsw-worker.js', { enabled: environment.production }),
    ChartsModule,
    ToastrModule.forRoot(),
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
