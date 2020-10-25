import { Component, Input, OnInit } from '@angular/core';
import { IBusinessRuleSingle } from '../../../domain/models/business-rules';
import { faSave, faTimesCircle } from '@fortawesome/free-solid-svg-icons';
import { FormBuilder } from '@angular/forms';

@Component({
  selector: 'app-admin-business-rule',
  templateUrl: './admin-business-rule.component.html',
  styleUrls: ['./admin-business-rule.component.scss']
})
export class AdminBusinessRuleComponent implements OnInit {

  @Input() businessRule: IBusinessRuleSingle;

  // Font Awseome icons
  public faCancel = faTimesCircle;
  public faSave = faSave;

  private initialValue: number;

  constructor(
    private fb: FormBuilder,
  ) { }

  ngOnInit(): void {
    this.initialValue = this.businessRule.value;
  }

  public onResetRule(): void {


  }

  public onSaveRule(): void {

  }

}
