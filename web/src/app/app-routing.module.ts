import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { HomeComponent } from './pages/home/home.component';
import { CatalogoComponent } from './pages/catalogo/catalogo.component';
import { AboutComponent } from './pages/about/about.component';
import { AutoresComponent } from './pages/autores/autores.component';

import { DeskboardGeneralComponent } from './pages/deskboard-general/deskboard-general.component';
import { FavoritosComponent } from './pages/favoritos/favoritos.component';
import { DashboardReportesComponent } from './pages/dashboard-reportes/dashboard-reportes.component';
import { DashboardGestionContratosPublicacionesComponent } from './pages/dashboard-gestion-contratos-publicaciones/dashboard-gestion-contratos-publicaciones.component';
import { DashboardReglasNegociosComponent } from './pages/dashboard-reglas-negocios/dashboard-reglas-negocios.component';
import { DashboardGestionAdminComponent } from './pages/dashboard-gestion-admin/dashboard-gestion-admin.component';

import { DevelopmentComponent } from './pages/development/development.component';
import { VisorComicComponent } from './pages/visor-comic/visor-comic.component';
import { NewPublicationComponent } from './components/deskboard/new-publication/new-publication.component';
import { AuthNotLoginGuard } from './guard/auth-not-login.guard';
import { PerfilComponent } from './pages/perfil/perfil.component';



const routes: Routes = [

  { path: 'development', component: DevelopmentComponent },
  { path: 'home', component: HomeComponent },
  { path: 'home/:id/recover-password/:temporal_password', component: HomeComponent, canActivate: [AuthNotLoginGuard] },
  { path: 'favorites', component: FavoritosComponent },
  { path: 'catalogue', component: CatalogoComponent },
  { path: 'authors', component:  AutoresComponent},
  { path: 'deskboard', component:  DeskboardGeneralComponent},
  { path: 'deskboard/publication/new', component:  NewPublicationComponent},
  { path: 'profile/:id', component: PerfilComponent},
  { path: 'profile/:id/editUser', component: PerfilComponent},
  { path: 'about', component: AboutComponent },
  { path: 'read/:id', component: VisorComicComponent },
  { path: 'dashboard-reportes', component: DashboardReportesComponent },
  { path: 'dashboard-publicaciones-contratos', component: DashboardGestionContratosPublicacionesComponent },
  { path: 'dashboard-reglas-negocio', component: DashboardReglasNegociosComponent },
  { path: 'dashboard-general', component: DashboardGestionAdminComponent },
  { path: '**', pathMatch: 'full', redirectTo: 'home' }

];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
