package data

import (
	"encoding/json"
	"fmt"

	models "github.com/CampbellKT95/Reading/server/models"
	"github.com/gomodule/redigo/redis"
)

func AddTweets(foundTweets models.TwitterPostLookup) interface{} {
	client := connectionPool.Get()
	defer client.Close()

	tweetsToAdd, err := json.Marshal(foundTweets.Data)
	if err != nil {
		panic(err)
	}

	tweets, err := client.Do("LPUSH", "tweets", tweetsToAdd)
	if err != nil {
		panic(err)
	}

	fmt.Println(tweets)
	return tweets
}

func FetchTweets() []string {
	client := connectionPool.Get()
	defer client.Close()

	tweets, err := client.Do("LRANGE", "tweets", 0, -1)
	if err != nil {
		panic(err)
	}

	tweetItems, err := redis.Strings(tweets, err)
	if err != nil {
		panic(err)
	}

	return tweetItems
}
