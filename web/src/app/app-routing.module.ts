import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { HomeComponent } from './pages/home/home.component';
import { CatalogoComponent } from './pages/catalogo/catalogo.component';
import { AboutComponent } from './pages/about/about.component';
import { AutoresComponent } from './pages/autores/autores.component';
import { DevelopmentComponent } from './pages/development/development.component';


const routes: Routes = [

  { path: 'development', component: DevelopmentComponent },
  { path: 'home', component: HomeComponent },
  { path: 'catalogue', component: CatalogoComponent },
  { path: 'authors', component:  AutoresComponent},
  { path: 'about', component: AboutComponent },
  { path: '**', pathMatch: 'full', redirectTo: 'home' }

];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
