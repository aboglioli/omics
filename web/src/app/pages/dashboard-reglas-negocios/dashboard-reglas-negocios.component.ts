import { Component, OnInit } from '@angular/core';
import { NgxSpinnerService } from 'ngx-spinner';
import { BusinessRulesService } from '../../domain/services/business-rules.service';
import { IBusinessRules, IBusinessRuleSingle } from '../../domain/models/business-rules';

@Component({
  selector: 'app-dashboard-reglas-negocios',
  templateUrl: './dashboard-reglas-negocios.component.html',
  styleUrls: ['./dashboard-reglas-negocios.component.scss']
})
export class DashboardReglasNegociosComponent implements OnInit {

  public businessRulesList: IBusinessRuleSingle[] = [];

  constructor(
    private spinnerService: NgxSpinnerService,
    private businessRulesService: BusinessRulesService
  ) { }

  ngOnInit(): void {

    this.spinnerService.show();
    this.businessRulesService.get().subscribe(

      (res: IBusinessRules) => {

        this.spinnerService.hide();

        for ( const key in res ) {

          if ( res.hasOwnProperty( key ) ) {

            this.businessRulesList.push({

              // Cambiar guión bajo por espacios y primera letra en mayúscula
              key,
              name: key.replace(/_/g, ' ').replace(/^./, key[0].toUpperCase()),
              value: res[key]

            });

          }

        }

        // console.log('TEST > ', this.businessRulesList);

      },
      (err: Error) => {

        this.spinnerService.hide();
        console.error(err);


      }

    );

  }

}
