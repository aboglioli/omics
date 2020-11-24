import { Component, OnInit } from '@angular/core';
import { faPercentage, faSave, faTimesCircle, faDollarSign } from '@fortawesome/free-solid-svg-icons';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { BreakpointObserver } from '@angular/cdk/layout';

import { PlanService } from '../../../domain/services/plan.service';
import { IPlan } from '../../../domain/models/plan';

@Component({
  selector: 'app-admin-manager-plan-card',
  templateUrl: './admin-manager-plan-card.component.html',
  styleUrls: ['./admin-manager-plan-card.component.scss']
})
export class AdminManagerPlanCardComponent implements OnInit {
  // Font Awseome icons
  public faCancel = faTimesCircle;
  public faSave = faSave;
  public faPercent = faPercentage;
  public faCurrency = faDollarSign;

  public form: FormGroup;

  public isBigScreen = true;

  public plan: IPlan;

  constructor(
    private planService: PlanService,
    private fb: FormBuilder,
    private breakpointObserver: BreakpointObserver
  ) { }

  ngOnInit(): void {
    this.checkWidthScreen();
    this.buildForm();
    this.loadPlan();
  }

  private loadPlan(): void {
    this.planService.getAll().subscribe(({plans}) => {
      if (plans.length > 0) {
        this.plan = plans[0];
      }
      this.form.setValue({ value: this.plan.price });
    });
  }

  private buildForm(): void {
    this.form = this.fb.group({
      value: [0, [Validators.required, Validators.min(0)]],
    });
  }

  public onResetPlan(): void {
    this.form.reset({
      value: this.plan.price,
    });
  }

  public onSavePlan(): void {
    const planPrice = this.form.get('value').value;

    this.planService
      .update(
        this.plan.id,
        {
          name: this.plan.name,
          description: this.plan.description,
          price: planPrice,
        },
      )
      .subscribe(() => {
        this.loadPlan();
      });
  }

  private checkWidthScreen(): void {
    this.breakpointObserver.observe(['(max-width: 950px)']).subscribe(
      (result: any) => {
        this.isBigScreen = (result.matches) ? false : true;
      });
  }

  get valorNoValido(): boolean {
    if (!this.form) {
      return true;
    }

    return this.form.get('value').invalid && this.form.get('value').touched;
  }
}
