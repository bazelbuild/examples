import { NgModule } from '@angular/core';
import { CommonModule } from 'common';
import { LibAComponent } from './lib-a.component';

@NgModule({
  declarations: [LibAComponent],
  imports: [CommonModule],
  exports: [LibAComponent],
})
export class LibAModule {}
