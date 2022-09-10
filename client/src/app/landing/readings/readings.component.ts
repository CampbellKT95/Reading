import { Component, OnInit, Input, ElementRef } from '@angular/core';
import { FormControl } from '@angular/forms';
import { Readings, JishoReturn, WordDescription } from 'src/app/shared/models/models';
import {SharedService} from '../../shared/service/shared.service';

@Component({
  selector: 'app-readings',
  templateUrl: './readings.component.html',
  styleUrls: ['./readings.component.css']
})
export class ReadingsComponent implements OnInit {

  constructor(private service: SharedService, private ele: ElementRef) { }

  ngOnInit(): void {
  }

  @Input() reading: Readings = {
    Text: '',
    Translation: ''
  }

  showTranslation: boolean = false;
  toggleTranslationDisplay() {
    this.showTranslation = !this.showTranslation
  }

  updatedTranslation = new FormControl('');
  updateTranslation() {
    this.service.updateReading(this.reading, this.updatedTranslation.value as string)
      .subscribe((result) => {
        // if successful, update translation in real-time
        if (result.status === 200) {
          const translation = this.ele.nativeElement.querySelector('.translation');

          translation.innerText = this.updatedTranslation.value
        }
      })
  }

  deleteReading() {
    this.service.deleteReading(this.reading)
      .subscribe((result) => {
        console.log(result);
      })
  }

  displayWordDescription: boolean = false;
  toggleDisplayWordDescription() {
    this.displayWordDescription = !this.displayWordDescription;
  }

  wordDescription: WordDescription[] = [];

  async fetchWordTranslation(event: any) {
    let savedWord: string = '';
    // find value within clipboard & save it
    await navigator.clipboard.readText().then(result => savedWord = result)
    // attempt to query Jisho with the saved word
    this.service.fetchWordTranslation(savedWord)
      .subscribe((result) => {
        const jishoReturn: JishoReturn = result.body as JishoReturn
        this.wordDescription = jishoReturn.Data as WordDescription[];

        this.toggleDisplayWordDescription()
      })
  }

}
