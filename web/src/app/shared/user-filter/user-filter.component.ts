import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import { ISearchCommand } from '../../domain/services/identity.service';
import { IRole } from '../../domain/models/user';
import { faFilter, faSearch, faTimesCircle } from '@fortawesome/free-solid-svg-icons';
import { FormBuilder, FormGroup } from '@angular/forms';

@Component({
  selector: 'app-user-filter',
  templateUrl: './user-filter.component.html',
  styleUrls: ['./user-filter.component.scss']
})
export class UserFilterComponent implements OnInit {

  @Output() OnFilterUser: EventEmitter<ISearchCommand> = new EventEmitter();

  @Input() rolesData: IRole[];

  // Font Awseome icons
  public faClear = faTimesCircle;
  public faFilter = faFilter;
  public faSearch = faSearch;

  public formFilter: FormGroup;

  constructor(
    private fb: FormBuilder,
  ) { }

  ngOnInit(): void {

    this.buildForms();

  }

  public buildForms(): void {

    this.formFilter = this.fb.group({
      name: [''],
      role: [''],
    });

  }

  public onClear(): void {

    this.formFilter.reset();
    this.onFilter();

  }

  public onFilter(): void {

    const searchUserCMD: ISearchCommand = {
      name: this.formFilter.get('name').value,
      role_id: this.formFilter.get('role').value
    };

    this.OnFilterUser.emit( searchUserCMD );

  }

}
