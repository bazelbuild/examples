import { ComponentFixture, TestBed } from '@angular/core/testing';

import { CommonComponent } from './common.component';

describe('CommonComponent', () => {
  let component: CommonComponent;
  let fixture: ComponentFixture<CommonComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [CommonComponent],
    }).compileComponents();

    fixture = TestBed.createComponent(CommonComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
