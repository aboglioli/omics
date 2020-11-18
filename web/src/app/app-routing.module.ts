// tslint:disable: max-line-length

import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { HomeComponent } from './pages/home/home.component';
import { CatalogoComponent } from './pages/catalogo/catalogo.component';
import { AboutComponent } from './pages/about/about.component';
import { AutoresComponent } from './pages/autores/autores.component';
import { NotificationsComponent } from './pages/notifications/notifications.component';

import { DeskboardGeneralComponent } from './pages/deskboard/deskboard-general/deskboard-general.component';
import { FavoritosComponent } from './pages/favoritos/favoritos.component';
import { DashboardReportesComponent } from './pages/dashboard-reportes/dashboard-reportes.component';
import { DashboardGestionContratosPublicacionesComponent } from './pages/dashboard-gestion-contratos-publicaciones/dashboard-gestion-contratos-publicaciones.component';
import { DashboardReglasNegociosComponent } from './pages/dashboard-reglas-negocios/dashboard-reglas-negocios.component';
import { DashboardGestionAdminComponent } from './pages/dashboard-gestion-admin/dashboard-gestion-admin.component';

import { DevelopmentComponent } from './pages/development/development.component';
import { VisorComicComponent } from './pages/visor-comic/visor-comic.component';
import { PublicationNewEditComponent } from './pages/deskboard/publication-new-edit/publication-new-edit.component';
import { AuthNotLoginGuard } from './guard/auth-not-login.guard';
import { PerfilComponent } from './pages/perfil/perfil.component';
import { AuthLoginGuard } from './guard/auth-login.guard';
import { PerfilEditarComponent } from './pages/perfil-editar/perfil-editar.component';
import { SameUserGuard } from './guard/same-user.guard';
import { PublicationOwnerGuard } from './guard/publication-owner.guard';
import { CollectionNewEditComponent } from './pages/deskboard/collection-new-edit/collection-new-edit.component';
import { CollectionOwnerGuard } from './guard/collection-owner.guard';
import { CollectionInfoComponent } from './pages/collection-info/collection-info.component';
import { PlansComponent } from './pages/plans/plans.component';
import { AdminGuard } from './guard/admin.guard';
import { ContentManagerGuard } from './guard/content-manager.guard';
import { DashboardRolesComponent } from './pages/dashboard-roles/dashboard-roles.component';


const routes: Routes = [

  { path: 'development', component: DevelopmentComponent },
  { path: 'home', component: HomeComponent },
  { path: 'home/:id/recover-password/:temporal_password', component: HomeComponent, canActivate: [AuthNotLoginGuard] },
  { path: 'notifications', component: NotificationsComponent, canActivate: [AuthLoginGuard] },
  { path: 'favorites', component: FavoritosComponent, canActivate: [AuthLoginGuard] },
  { path: 'catalogue', component: CatalogoComponent },
  { path: 'authors', component:  AutoresComponent},
  { path: 'deskboard', component:  DeskboardGeneralComponent, canActivate: [AuthLoginGuard]},
  { path: 'deskboard/publication/new', component:  PublicationNewEditComponent},
  { path: 'deskboard/publication/edit/:id', component:  PublicationNewEditComponent, canActivate: [PublicationOwnerGuard]},
  { path: 'deskboard/collection/new', component:  CollectionNewEditComponent},
  { path: 'deskboard/collection/edit/:id', component:  CollectionNewEditComponent, canActivate: [CollectionOwnerGuard]},
  { path: 'profile/:id', component: PerfilComponent },
  { path: 'profile/:id/editUser', component: PerfilEditarComponent,  canActivate: [AuthLoginGuard, SameUserGuard]},
  { path: 'collection/:id', component: CollectionInfoComponent },
  { path: 'about', component: AboutComponent },
  { path: 'read/:id', component: VisorComicComponent, canActivate: [AuthLoginGuard] },
  { path: 'dashboard-reportes', component: DashboardReportesComponent, canActivate: [AdminGuard] },
  { path: 'dashboard-publicaciones-contratos', component: DashboardGestionContratosPublicacionesComponent, canActivate: [ContentManagerGuard] },
  { path: 'dashboard-reglas-negocio', component: DashboardReglasNegociosComponent, canActivate: [AdminGuard]  },
  { path: 'dashboard-general', component: DashboardGestionAdminComponent, canActivate: [AuthLoginGuard, AdminGuard ] },
  { path: 'dashboard-roles', component: DashboardRolesComponent, canActivate: [AuthLoginGuard, AdminGuard ] },
  { path: 'plans', component: PlansComponent, canActivate: [AuthLoginGuard] },
  { path: '**', pathMatch: 'full', redirectTo: 'home' },

];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
