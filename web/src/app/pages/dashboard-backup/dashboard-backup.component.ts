import { Component, OnInit } from '@angular/core';
import { NgxSpinnerService } from 'ngx-spinner';
import { map } from 'rxjs/operators';
import { faPlusCircle } from '@fortawesome/free-solid-svg-icons';
import { BusinessRulesService } from '../../domain/services/business-rules.service';
import { IBusinessRules, IBusinessRuleSingle, IBusinessType } from '../../domain/models/business-rules';
import { SweetAlertGenericMessageService } from 'src/app/services/sweet-alert-generic-message.service';
import { TypeAmount } from 'src/app/models/enums.model';
import { BackupService } from '../../domain/services/backup.service';
import { IBackupFile } from '../../domain/models/backup';


@Component({
  selector: 'app-dashboard-backup',
  templateUrl: './dashboard-backup.component.html',
  styleUrls: ['./dashboard-backup.component.scss']
})
export class DashboardBackup implements OnInit {
  // Font Awseome icons
  public faAdd = faPlusCircle;

  public backups: IBackupFile[];

  constructor(
    private spinnerService: NgxSpinnerService,
    private sweetAlertGenericService: SweetAlertGenericMessageService,
    private backupService: BackupService,
  ) { }

  ngOnInit(): void {
    this.loadBackups();
  }

  private loadBackups(): void {
    this.spinnerService.show();
    this.backupService
      .list()
      .pipe(
        map((backupFiles) =>
          backupFiles
            .map((backup) => {
              const timestamp = +backup.file.split('_')[0];
              const date = new Date(timestamp * 1000);

              return {
                ...backup,
                date,
                timestamp,
              };
            })
            .sort((a, b) => b.timestamp - a.timestamp)
        ),
      )
      .subscribe((backupFiles) => {
        this.spinnerService.hide();
        this.backups = backupFiles;
      },
    );
  }

  public generate(): void {
    this.spinnerService.show();
    this.backupService.generate().subscribe(
      () => {
        this.spinnerService.hide();
        this.loadBackups();
      },
    );
  }

}
