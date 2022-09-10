package tools

import (
	"encoding/json"
	"io"
	"net/http"
	"os"

	data "github.com/CampbellKT95/Reading/server/database"
	models "github.com/CampbellKT95/Reading/server/models"
	"github.com/gin-gonic/gin"
)

// docs: https://developer.twitter.com/en/docs/twitter-api/tweets/timelines/api-reference/get-users-id-tweets
// endpoint: https://api.twitter.com/2/users/:id/tweets

var foundTweets models.TwitterPostLookup

func AddNewTweets(conn *gin.Context) {
	// must look up user Id first
	userId := LookUpUserId()
	client := &http.Client{}

	req, err := http.NewRequest("GET", "https://api.twitter.com/2/users/"+userId+"/tweets", nil)
	if err != nil {
		panic(err)
	}
	req.Header.Add("Authorization", "Bearer "+os.Getenv("TWITTER_API_BEARER_READING"))

	response, err := client.Do(req)
	if err != nil {
		panic(err)
	}

	body, err := io.ReadAll(response.Body)
	if err != nil {
		panic(err)
	}

	json.Unmarshal(body, &foundTweets)

	// with tweets now in foundTweets, push to redis
	data.AddTweets(foundTweets)
}

var UserId models.UserLookUpId

func LookUpUserId() string {
	// http.Get does not allow headers, so need to utilize client instead
	client := &http.Client{}
	req, err := http.NewRequest("GET", "https://api.twitter.com/2/users/by?usernames=rikakun_muku", nil)
	if err != nil {
		panic(err)
	}

	req.Header.Add("Authorization", "Bearer "+os.Getenv("TWITTER_API_BEARER_READING"))

	response, err := client.Do(req)
	if err != nil {
		panic(err)
	}
	defer response.Body.Close()

	body, err := io.ReadAll(response.Body)
	if err != nil {
		panic(err)
	}

	json.Unmarshal(body, &UserId)

	userId := UserId.Data[0].Id
	return userId
}

func FetchTweets(conn *gin.Context) {
	result := data.FetchTweets()

	tweets, err := json.Marshal(result)
	if err != nil {
		panic(err)
	}

	conn.Data(http.StatusOK, "application/json", tweets)
}
