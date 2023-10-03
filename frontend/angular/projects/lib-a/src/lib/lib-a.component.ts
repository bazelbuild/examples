import { Component } from '@angular/core';

@Component({
  selector: 'example-library',
  template: `
    <strong> The library component! With <lib-common></lib-common> </strong>
  `,
  styleUrls: ['./lib-a.component.css'],
})
export class LibAComponent {}
