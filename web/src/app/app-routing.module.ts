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



const routes: Routes = [

  { path: 'development', component: DevelopmentComponent },
  { path: 'home', component: HomeComponent },
  { path: 'favorites', component: FavoritosComponent },
  { path: 'catalogue', component: CatalogoComponent },
  { path: 'authors', component:  AutoresComponent},
  { path: 'deskboard', component:  DeskboardGeneralComponent},
  { path: 'about', component: AboutComponent },
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
