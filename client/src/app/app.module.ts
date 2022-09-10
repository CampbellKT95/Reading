import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { HttpClientModule } from '@angular/common/http';
import { AppComponent } from './app.component';
import { LandingComponent } from './landing/landing.component';
import { ReadingsComponent } from './landing/readings/readings.component';
import { ReactiveFormsModule } from '@angular/forms';
import { TweetsComponent } from './landing/tweets/tweets.component';

@NgModule({
  declarations: [
    AppComponent,
    LandingComponent,
    ReadingsComponent,
    TweetsComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    HttpClientModule,
    ReactiveFormsModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
