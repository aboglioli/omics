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
import { DashboardBackup } from './pages/dashboard-backup/dashboard-backup.component';

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
import { DashboardRolesComponent } from './pages/dashboard-roles/dashboard-roles.component';
import { PermissionAnyGuard } from './guard/permission-any.guard';


const routes: Routes = [

  { path: 'development', component: DevelopmentComponent },
  { path: 'home', component: HomeComponent },
  { path: 'home/:id/recover-password/:temporal_password', component: HomeComponent, canActivate: [AuthNotLoginGuard] },
  { path: 'notifications', component: NotificationsComponent, canActivate: [AuthLoginGuard, PermissionAnyGuard], data: {permission: ['get_notifications']} },
  { path: 'favorites', component: FavoritosComponent, canActivate: [AuthLoginGuard, PermissionAnyGuard], data: { permission: ['get_reader_favorites'] } },
  { path: 'catalogue', component: CatalogoComponent },
  { path: 'authors', component:  AutoresComponent},
  { path: 'deskboard', component:  DeskboardGeneralComponent, canActivate: [AuthLoginGuard]},
  { path: 'deskboard/publication/new', component:  PublicationNewEditComponent},
  { path: 'deskboard/publication/edit/:id', component:  PublicationNewEditComponent, canActivate: [PublicationOwnerGuard, PermissionAnyGuard], data: {permission: ['update_publication']} },
  { path: 'deskboard/collection/new', component:  CollectionNewEditComponent, canActivate: [AuthLoginGuard, PermissionAnyGuard], data: {permission: ['create_collection']} },
  { path: 'deskboard/collection/edit/:id', component:  CollectionNewEditComponent, canActivate: [CollectionOwnerGuard, PermissionAnyGuard], data: { permission: ['update_collection'] }},
  { path: 'profile/:id', component: PerfilComponent, canActivate: [PermissionAnyGuard], data: { permission: ['get_reader'] } },
  { path: 'profile/:id/editUser', component: PerfilEditarComponent,  canActivate: [AuthLoginGuard, SameUserGuard, PermissionAnyGuard], data: {permission: ['update_own_user', 'subscribe', 'delete_own_user', 'change_user_password']}},
  { path: 'collection/:id', component: CollectionInfoComponent },
  { path: 'about', component: AboutComponent },
  { path: 'read/:id', component: VisorComicComponent, canActivate: [AuthLoginGuard] },
  { path: 'dashboard-reportes', component: DashboardReportesComponent, canActivate: [AuthLoginGuard, PermissionAnyGuard], data: {permission: ['generate_report']  } },
  { path: 'dashboard-publicaciones-contratos', component: DashboardGestionContratosPublicacionesComponent, canActivate: [PermissionAnyGuard], data: { permission: ['approve_reject_contract', 'approve_reject_publication'] } },
  { path: 'dashboard-reglas-negocio', component: DashboardReglasNegociosComponent, canActivate: [AuthLoginGuard, PermissionAnyGuard], data: { permission: ['change_business_rules'] }  },
  { path: 'dashboard-general', component: DashboardGestionAdminComponent, canActivate: [AuthLoginGuard, PermissionAnyGuard ], data: { permission: ['create_category', 'update_category', 'delete_category'] } },
  { path: 'dashboard-roles', component: DashboardRolesComponent, canActivate: [AuthLoginGuard,  PermissionAnyGuard], data: { permission: ['get_permissions', 'get_any_user'] } },
  { path: 'dashboard-backups', component: DashboardBackup, canActivate: [AuthLoginGuard, PermissionAnyGuard], data: {permission: ['generate_backup']  } },
  { path: 'plans', component: PlansComponent, canActivate: [AuthLoginGuard, PermissionAnyGuard], data: {permission: ['subscribe'] } },
  { path: '**', pathMatch: 'full', redirectTo: 'home' },

];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
