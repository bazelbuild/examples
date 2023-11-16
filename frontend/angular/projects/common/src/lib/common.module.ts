import { NgModule } from '@angular/core';
import { CommonComponent } from './common.component';

@NgModule({
  declarations: [CommonComponent],
  exports: [CommonComponent],
})
export class CommonModule {}
