import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import { IBusinessRuleSingle, IBusinessType } from '../../../domain/models/business-rules';
import { faPercentage, faSave, faTimesCircle, faDollarSign } from '@fortawesome/free-solid-svg-icons';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { TypeAmount } from 'src/app/models/enums.model';
import { BreakpointObserver } from '@angular/cdk/layout';

@Component({
  selector: 'app-admin-business-rule',
  templateUrl: './admin-business-rule.component.html',
  styleUrls: ['./admin-business-rule.component.scss']
})
export class AdminBusinessRuleComponent implements OnInit {

  @Input() businessRule: IBusinessRuleSingle;
  @Output() OnRuleChange: EventEmitter<IBusinessType> = new EventEmitter();

  // Font Awseome icons
  public faCancel = faTimesCircle;
  public faSave = faSave;
  public faPercent = faPercentage;
  public faCurrency = faDollarSign;

  public formRule: FormGroup;
  private initialValue: number;

  public isBigScreen = true;

  constructor(
    private fb: FormBuilder,
    private breakpointObserver: BreakpointObserver
  ) { }

  ngOnInit(): void {

    this.checkWidthScreen();

    this.formBuild();

  }

  private formBuild(): void {

    this.initialValue = this.businessRule.value;

    switch ( this.businessRule.type ) {

      case TypeAmount.percent: {

        this.initialValue *= 100;

        this.formRule = this.fb.group({
          value: [ this.initialValue, [ Validators.required, Validators.min(0), Validators.max(100) ] ]
        });
        break;
      }

      case TypeAmount.normal:
      case TypeAmount.currency: {

        this.formRule = this.fb.group({
          value: [ this.initialValue, [ Validators.required, Validators.min(0) ] ]
        });
        break;
      }

    }

  }

  public onResetRule(): void {
    this.formRule.reset({
      value: this.initialValue
    });
  }

  public onSaveRule(): void {

    this.businessRule.value = this.formRule.get('value').value;

    if ( this.businessRule.type === TypeAmount.percent ) {
      this.businessRule.value  = Number( (this.businessRule.value * 0.01).toFixed(2) );
      this.initialValue = this.businessRule.value;
    }

    this.OnRuleChange.emit( this.businessRule );

    this.initialValue =  (this.businessRule.type === TypeAmount.percent) ?
      ( this.businessRule.value * 100 ) : this.businessRule.value;

  }

  private checkWidthScreen(): void {

    this.breakpointObserver.observe(['(max-width: 950px)']).subscribe(
      (result: any) => {

        this.isBigScreen = (result.matches) ? false : true;

      });
  }

  // Getters
  get valorNoValido(): boolean {
    return ( this.formRule.get('value').invalid && this.formRule.get('value').touched );
  }

}
