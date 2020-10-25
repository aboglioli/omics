import { Component, Input, OnInit } from '@angular/core';
import { IBusinessRuleSingle } from '../../../domain/models/business-rules';

@Component({
  selector: 'app-admin-business-rule',
  templateUrl: './admin-business-rule.component.html',
  styleUrls: ['./admin-business-rule.component.scss']
})
export class AdminBusinessRuleComponent implements OnInit {

  @Input() businessRule: IBusinessRuleSingle;

  constructor() { }

  ngOnInit(): void {
  }

}
