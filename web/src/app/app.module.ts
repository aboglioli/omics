import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { FormsModule, ReactiveFormsModule } from '@angular/forms'; // Evita el refresh en formularios y manejado por angular
import { environment } from '../environments/environment';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';

import { HttpClientModule  } from '@angular/common/http';

import { ServiceWorkerModule } from '@angular/service-worker'; // PWA dependencia
import { ToastrModule } from 'ngx-toastr';
import { ChartsModule } from 'ng2-charts';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { NgxSpinnerModule } from 'ngx-spinner';
import { SweetAlert2Module } from '@sweetalert2/ngx-sweetalert2';
import { BarRatingModule } from 'ngx-bar-rating';
import { CustomPaginator } from './shared/custom-instances/CustomPaginatorConfiguration';

// Modulos auxiliares
import { MaterialModule } from './auxiliar-modules/material.module';
import { AngularBootstrapModule } from './auxiliar-modules//angular-bootstrap.module';

// Componentes
import { HomeComponent } from './pages/home/home.component';
import { AboutComponent } from './pages/about/about.component';
import { LoginRegisterComponent } from './components/user/login-register/login-register.component';
import { VisorComicComponent } from './pages/visor-comic/visor-comic.component';
import { FavoritosComponent } from './pages/favoritos/favoritos.component';
import { PerfilComponent } from './pages/perfil/perfil.component';
import { SuscripcionComponent } from './components/suscripcion/suscripcion.component';
import { DonacionComponent } from './components/donacion/donacion.component';
import { DeskboardGeneralComponent } from './pages/deskboard/deskboard-general/deskboard-general.component';
import { DashboardReportesComponent } from './pages/dashboard-reportes/dashboard-reportes.component';
import { DashboardGestionAdminComponent } from './pages/dashboard-gestion-admin/dashboard-gestion-admin.component';
import { DashboardGestionContratosPublicacionesComponent } from './pages/dashboard-gestion-contratos-publicaciones/dashboard-gestion-contratos-publicaciones.component';
import { DashboardReglasNegociosComponent } from './pages/dashboard-reglas-negocios/dashboard-reglas-negocios.component';
import { DashboardBackup } from './pages/dashboard-backup/dashboard-backup.component';
import { CatalogoComponent } from './pages/catalogo/catalogo.component';
import { PublicationInfoComponent } from './components/publication/publication-info/publication-info.component';
import { NavBarComponent } from './shared/nav-bar/nav-bar.component';
import { SideNavMenuMainComponent } from './shared/side-nav-menu-main/side-nav-menu-main.component';
import { AutoresComponent } from './pages/autores/autores.component';
import { UnderConstructionComponent } from './shared/under-construction/under-construction.component';
import { DeskboardMisComicsComponent } from './components/deskboard/deskboard-mis-comics/deskboard-mis-comics.component';
import { PublicationCardManagerComponent } from './components/publication-card-type/publication-card/publication-card-manager.component';
import { NotificationsComponent } from './pages/notifications/notifications.component';
import { PlansComponent } from './pages/plans/plans.component';
// --->

// Development
import { DevelopmentComponent } from './pages/development/development.component';
import { DevUploadFileComponent } from './pages/development/upload-file/upload-file';
import { DevNewPublicationComponent } from './pages/development/new-publication/new-publication';
import { DevPaymentComponent } from './pages/development/payment/payment';
import { DevReportsComponent } from './pages/development/reports/reports';

// Domain
import { DomainModule } from './domain/module';
import { PasswordForgotComponent } from './components/user/password-recovery/password-forgot/password-forgot.component';
import { PasswordRewriteComponent } from './components/user/password-recovery/password-rewrite/password-rewrite.component';
import { SideNavMenuUserComponent } from './shared/side-nav-menu-user/side-nav-menu-user.component';
import { PerfilEditarComponent } from './pages/perfil-editar/perfil-editar.component';
import { PublicationApproveRejectMotiveComponent } from './components/dashboard/publication-approve-reject-motive/publication-approve-reject-motive.component';
import { PublicationCardAuthorComponent } from './components/publication-card-type/publication-card-author/publication-card-author.component';
import { PublicationCardReaderComponent } from './components/publication-card-type/publication-card-reader/publication-card-reader.component';
import { TruncateStringPipe } from './pipes/truncate-string.pipe';
import { PublicationReviewAddComponent } from './components/publication/publication-review-add/publication-review-add.component';
import { PublicationReviewsBoxComponent } from './components/publication/publication-reviews-box/publication-reviews-box.component';
import { DeskboardReportComponent } from './components/deskboard/deskboard-reports/deskboard-report.component';
import { DeskboardWalletComponent } from './components/deskboard/deskboard-wallet/deskboard-wallet.component';
import { CollectionCardReaderComponent } from './components/collection-card-type/collection-card-reader/collection-card-reader.component';
import { CollectionCardAuthorComponent } from './components/collection-card-type/collection-card-author/collection-card-author.component';
import { PublicationNewEditComponent } from './pages/deskboard/publication-new-edit/publication-new-edit.component';
import { CollectionNewEditComponent } from './pages/deskboard/collection-new-edit/collection-new-edit.component';
import { CollectionInfoComponent } from './pages/collection-info/collection-info.component';
import { CatalogueFilterComponent } from './components/catalogue-filter/catalogue-filter.component';
import { AdminCategoriesComponent } from './components/dashboard/admin-categories/admin-categories.component';
import { AdminTagsComponent } from './components/dashboard/admin-tags/admin-tags.component';
import { AdminBusinessRuleComponent } from './components/dashboard/admin-business-rule/admin-business-rule.component';
import { DeskboardMedioCobroComponent } from './components/deskboard/deskboard/deskboard-medio-cobro.component';
import { DashboardRolesComponent } from './pages/dashboard-roles/dashboard-roles.component';
import { UserRolesListComponent } from './components/dashboard/dashboard-roles/user-roles/user-roles-list/user-roles-list.component';
import { UserRolesEditComponent } from './components/dashboard/dashboard-roles/user-roles/user-roles-edit/user-roles-edit.component';
import { RolesManagerListComponent } from './components/dashboard/dashboard-roles/roles-manager/roles-manager-list/roles-manager-list.component';
import { RolesManagerEditComponent } from './components/dashboard/dashboard-roles/roles-manager/roles-manager-edit/roles-manager-edit.component';
import { MatPaginatorIntl } from '@angular/material/paginator';
import { UserFilterComponent } from './shared/user-filter/user-filter.component';
import { AdminManagerPlanCardComponent } from './components/dashboard/admin-manager-plan-card/admin-manager-plan-card.component';
import { TermsConditionsTextComponent } from './components/user/terms-conditions-text/terms-conditions-text.component';


@NgModule({
  declarations: [
    AppComponent,
    HomeComponent,
    AboutComponent,
    LoginRegisterComponent,
    VisorComicComponent,
    FavoritosComponent,
    PerfilComponent,
    SuscripcionComponent,
    DonacionComponent,
    DeskboardGeneralComponent,
    DashboardReportesComponent,
    DashboardGestionAdminComponent,
    DashboardGestionContratosPublicacionesComponent,
    DashboardReglasNegociosComponent,
    DashboardBackup,
    CatalogoComponent,
    PublicationInfoComponent,
    NavBarComponent,
    SideNavMenuMainComponent,
    AutoresComponent,
    UnderConstructionComponent,
    DeskboardMisComicsComponent,
    PublicationNewEditComponent,
    PasswordForgotComponent,
    PasswordRewriteComponent,
    PublicationCardManagerComponent,
    NotificationsComponent,
    PlansComponent,

    DevelopmentComponent,
    DevUploadFileComponent,
    DevNewPublicationComponent,
    DevPaymentComponent,
    DevReportsComponent,
    SideNavMenuUserComponent,
    PerfilEditarComponent,
    PublicationApproveRejectMotiveComponent,
    PublicationCardAuthorComponent,
    PublicationCardReaderComponent,
    TruncateStringPipe,
    PublicationReviewAddComponent,
    PublicationReviewsBoxComponent,
    DeskboardReportComponent,
    DeskboardWalletComponent,
    CollectionCardReaderComponent,
    CollectionCardAuthorComponent,
    CollectionNewEditComponent,
    CollectionInfoComponent,
    CatalogueFilterComponent,
    AdminCategoriesComponent,
    AdminTagsComponent,
    AdminBusinessRuleComponent,
    DeskboardMedioCobroComponent,
    DashboardRolesComponent,
    UserRolesListComponent,
    UserRolesEditComponent,
    RolesManagerListComponent,
    RolesManagerEditComponent,
    UserFilterComponent,
    AdminManagerPlanCardComponent,
    TermsConditionsTextComponent,
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    ReactiveFormsModule, // si se usa reactivos
    FormsModule, // Si se usa por template
    HttpClientModule,
    ServiceWorkerModule.register('ngsw-worker.js', { enabled: environment.production }),
    ChartsModule,
    ToastrModule.forRoot(),
    FontAwesomeModule,
    NgxSpinnerModule,
    SweetAlert2Module.forRoot(),
    BarRatingModule,

    // Custom modules
    DomainModule,
    MaterialModule,
    AngularBootstrapModule,
  ],
  providers: [
    { provide: MatPaginatorIntl, useValue: CustomPaginator() }
  ],
  bootstrap: [AppComponent]
})
export class AppModule { }
