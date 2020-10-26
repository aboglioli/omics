import { Component, OnInit } from '@angular/core';
import { NgxSpinnerService } from 'ngx-spinner';
import { BusinessRulesService } from '../../domain/services/business-rules.service';
import { IBusinessRules, IBusinessRuleSingle, IBusinessType } from '../../domain/models/business-rules';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';
import { TypeAmount } from 'src/app/models/enums.model';


@Component({
  selector: 'app-dashboard-reglas-negocios',
  templateUrl: './dashboard-reglas-negocios.component.html',
  styleUrls: ['./dashboard-reglas-negocios.component.scss']
})
export class DashboardReglasNegociosComponent implements OnInit {

  public businessRulesList: IBusinessRuleSingle[] = [];

  constructor(
    private spinnerService: NgxSpinnerService,
    private businessRulesService: BusinessRulesService,
    private sweetAlertGenericService: SweetAlertGenericMessageService,
  ) { }

  ngOnInit(): void {

    this.spinnerService.show();
    this.businessRulesService.get().subscribe(

      (res: IBusinessRules) => {

        this.spinnerService.hide();

        for ( const key in res ) {

          if ( res.hasOwnProperty( key ) ) {

            const nameTypeAux = this.getNameByKey( key );
            this.businessRulesList.push({

              // Cambiar guión bajo por espacios y primera letra en mayúscula
              key,
              name: nameTypeAux.name,
              value: res[key],
              type: nameTypeAux.type

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


  private getNameByKey( key: string ): IBusinessType {

    switch ( key ) {

      case 'days_to_generate_summaries': {
        return {
          name: 'Días para generar resumen',
          type: TypeAmount.normal
        };
      }

      case 'donation_percentage_retention': {
        return {
          name: 'Retención porcentual de donaciones',
          type: TypeAmount.percent
        };
      }

      case 'minimum_charge_amount': {
        return {
          name: 'Monto mínomo para cobrar',
          type: TypeAmount.currency
        };
      }

      case 'minimum_donation_amount': {
        return {
          name: 'Monto mínimo donación',
          type: TypeAmount.currency
        };
      }

      case 'minimum_views_percentage_to_require_contract': {
        return {
          name: 'Porcentaje mínimo de vistas para solicitar contrato',
          type: TypeAmount.percent
        };
      }

      case 'subscription_percentage_retention': {
        return {
          name: 'Porcentaje de retención de suscripción',
          type: TypeAmount.percent
        };
      }

    }


  }

  public saveRule( rule: IBusinessRuleSingle ): void {

    console.log('TEST > ', rule);

  }

}
