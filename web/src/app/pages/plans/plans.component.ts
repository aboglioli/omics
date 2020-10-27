import { Component, OnInit, OnDestroy } from '@angular/core';
import { Router } from '@angular/router';

import { IPlan } from '../../domain/models';
import { PlanService } from '../../domain/services/plan.service';
import { ReaderService } from '../../domain/services/reader.service';

@Component({
  selector: 'app-plans',
  templateUrl: './plans.component.html',
  styleUrls: ['./plans.component.scss']
})
export class PlansComponent implements OnInit, OnDestroy {
  public plans: IPlan[];
  public readerPlanSubscription: string;
  private lastInterval: number;

  constructor(
    private router: Router,
    private planService: PlanService,
    private readerService: ReaderService,
  ) { }

  ngOnInit(): void {
    this.planService.getAll().subscribe(
      (res) => {
        this.plans = res.plans;
      },
      (err) => {
        console.log(err);
      },
    );

    this.readerService.getSubscription('me').subscribe(
      (res) => {
        if (res.status.status === 'active') {
          this.readerPlanSubscription = res.plan.id;
        }
      }
    );
  }

  ngOnDestroy(): void {
    clearInterval(this.lastInterval);
  }

  subscribe(plan: IPlan): void {
    clearInterval(this.lastInterval);

    this.planService.subscribe(plan.id).subscribe(
      (res) => {
        window.open(res.payment_link);
        this.lastInterval = setInterval(
          () => {
            this.readerService.getSubscription('me').subscribe(
              (res) => {
                if (res.status.status === 'active') {
                  clearInterval(this.lastInterval);
                  this.router.navigate(['catalogue']);
                }
              },
              (err) => {
                clearInterval(this.lastInterval);
                console.log(err);
              }
            );
          },
          2000,
        );
      },
      (err) => {
        console.log(err);
      }
    )
  }

}
