import { Injectable } from '@angular/core';

@Injectable()
export class ConfigService {
  constructor() { }

  public baseUrl(): string {
    return 'http://localhost:3000/api';
  }
}
