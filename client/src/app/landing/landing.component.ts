import { Component, OnInit } from '@angular/core';
import {SharedService} from "../shared/service/shared.service";
import { Readings } from '../shared/models/models';
import { FormControl } from '@angular/forms';

@Component({
  selector: 'app-landing',
  templateUrl: './landing.component.html',
  styleUrls: ['./landing.component.css']
})

export class LandingComponent implements OnInit {

  constructor(private service: SharedService) { }

  ngOnInit(): void {
    this.fetchAllReadings();
  }

  allReadings: Readings[] = []
  fetchAllReadings() {
    this.service.fetchAllReadings()
      .subscribe((result) => {
        const response: string[] = result.body as string[]

        response.forEach((reading) => {
          const parsedResult = JSON.parse(reading)
          this.allReadings.push(parsedResult)
        })
      })

      console.log(this.allReadings)
  }

  addReadingModal = false;
  toggleAddReadingModal(event: any, target: string) {
    event.stopPropagation();
    
    if (target === "button") {
      this.addReadingModal = !this.addReadingModal
    } else {
      if (this.addReadingModal === true && event.target.classList.contains("background-add-reading-modal")) {
        this.addReadingModal = false;
      }
    }
  }

  newReading = new FormControl('');
  addReading() {

    const readingToAdd: Readings = {
      Text: this.newReading.value as string,
      Translation: ''
    }

    this.service.addReading(readingToAdd)
      .subscribe((result) => {
        console.log(result)
      })
  }
}
