import { Component, OnInit } from '@angular/core';
import {SharedService} from "../../shared/service/shared.service";
import {Tweet} from "../../shared/models/models";

@Component({
  selector: 'app-tweets',
  templateUrl: './tweets.component.html',
  styleUrls: ['./tweets.component.css']
})
export class TweetsComponent implements OnInit {

  constructor(private service: SharedService) { }

  ngOnInit(): void {
    // this.fetchTweets()
  }

  // need to update the type of allTweets to a model that mirrors result
  allTweets: Tweet[] = []
  fetchTweets() {
    this.service.fetchTweets()
      .subscribe((result: any) => {
        const jsonTweets: string = result.body[0]
        this.allTweets = JSON.parse(jsonTweets);
      })
  }
}
