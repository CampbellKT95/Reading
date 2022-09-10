package data

import (
	"encoding/json"

	models "github.com/CampbellKT95/Reading/server/models"
	"github.com/gomodule/redigo/redis"
)

var connectionPool = DbConnect()

func DbConnect() *redis.Pool {

	return &redis.Pool{
		MaxIdle:   80,
		MaxActive: 12000,
		Dial: func() (redis.Conn, error) {
			connection, err := redis.Dial("tcp", redisUrl, redis.DialPassword(redisPass))
			if err != nil {
				panic(err.Error())
			}
			return connection, err
		},
	}
}

func FetchLibrary() interface{} {
	client := connectionPool.Get()
	defer client.Close()

	var library interface{}
	// LRANGE requires start & end row of the list
	library, err := client.Do("LRANGE", "library", 0, -1)
	if err != nil {
		panic(err)
	}

	// need to use "reply helper functions" or type assertion to transform it out of an interface{}
	readingItems, err := redis.Strings(library, err)
	if err != nil {
		panic(err)
	}

	return readingItems
}

// convert item to json & append to a list
func AddToLibrary(newLibraryItem []byte) interface{} {
	client := connectionPool.Get()
	defer client.Close()

	addedItem, err := client.Do("LPUSH", "library", newLibraryItem)
	if err != nil {
		panic(err)
	}

	return addedItem
}

func FindItemIndex(client redis.Conn, itemToFind []byte) int {
	// in order to find the index of an item, it needs to be an EXACT match
	foundItem, err := client.Do("LPOS", "library", itemToFind)
	if err != nil {
		panic(err)
	}

	itemIndex, err := redis.Int(foundItem, err)
	if err != nil {
		panic(err)
	}

	return itemIndex
}

var updateRequest models.UpdateRequest

func UpdateLibraryItem(request []byte) interface{} {
	client := connectionPool.Get()
	defer client.Close()

	// unmarshal to place request content into proper model
	err := json.Unmarshal(request, &updateRequest)
	if err != nil {
		panic(err)
	}

	// re-marshal the 'original' text model (since it is placed in redis serialized)
	itemToFind, err := json.Marshal(updateRequest.Original)
	if err != nil {
		panic(err)
	}

	var itemIndex int = FindItemIndex(client, itemToFind)

	//re-marshal updated text
	itemToUpdate, err := json.Marshal(updateRequest.Update)
	if err != nil {
		panic(err)
	}

	updatedItem, err := client.Do("LSET", "library", itemIndex, itemToUpdate)
	if err != nil {
		panic(err)
	}

	return updatedItem
}

func DeleteLibraryItem(itemToDelete []byte) interface{} {
	client := connectionPool.Get()
	defer client.Close()

	deletedItem, err := client.Do("LREM", "library", 1, itemToDelete)
	if err != nil {
		panic(err)
	}

	return deletedItem
}
