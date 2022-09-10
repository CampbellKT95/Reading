import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Readings } from '../models/models';

@Injectable({
  providedIn: 'root'
})

export class SharedService {

  constructor(private http: HttpClient) { }

  fetchAllReadings() {
    return this.http.get(`http://localhost:8080/library`, {
      observe: "response"
    })
  }

  addReading(newReading: Readings) {
    return this.http.post(`http://localhost:8080/library`, JSON.stringify(newReading),{
      observe: "response"
    })
  }

  // need to update with new params being passed
  updateReading(original: Readings, updatedTranslation: string) {

    const updatedReading = {
      Text: original.Text,
      Translation: updatedTranslation
    }

    const replacement = {
      Original: original,
      Update: updatedReading
    }

    return this.http.put("http://localhost:8080/library", JSON.stringify(replacement), {
      observe: "response"
    })
  }

  deleteReading(readingToDelete: Readings) {
    return this.http.delete("http://localhost:8080/library", {
      observe: "response",
      body: JSON.stringify(readingToDelete)
    })
  }

  fetchWordTranslation(word: string) {
    return this.http.get(`http://localhost:8080/tools/translation/${word}`, {
      observe: "response"
    })
  }

  fetchTweets() {
    return this.http.get("http://localhost:8080/tools/twitter/fetch", {
      observe: "response"
    })
  }

}
