package api

import (
	"encoding/json"
	"io/ioutil"
	"log"
	"net/http"

	data "github.com/CampbellKT95/Reading/server/database"
	"github.com/gin-gonic/gin"
)

func FetchLibrary(conn *gin.Context) {
	result := data.FetchLibrary()

	response, responseError := json.Marshal(result)
	if responseError != nil {
		log.Fatal(responseError)
	}

	conn.Data(http.StatusOK, "application/json", response)
}

func AddToLibrary(conn *gin.Context) {
	requestBody, err := ioutil.ReadAll(conn.Request.Body)
	if err != nil {
		log.Fatal(err)
	}

	// need to leave the data serialized so that it remains in json within redis
	result := data.AddToLibrary(requestBody)

	response, responseError := json.Marshal(result)
	if responseError != nil {
		log.Fatal(responseError)
	}
	conn.Data(http.StatusOK, "application/json", response)
}

func UpdateLibraryItem(conn *gin.Context) {
	// in order to get the actual index, need to pass the EXACT matching string (ie the sentence & translation)
	requestBody, err := ioutil.ReadAll(conn.Request.Body)
	if err != nil {
		log.Fatal(err)
	}

	result := data.UpdateLibraryItem(requestBody)

	response, responseError := json.Marshal(result)
	if responseError != nil {
		log.Fatal(responseError)
	}
	conn.Data(http.StatusOK, "application/json", response)
}

// removes exact match from list
func DeleteFromLibrary(conn *gin.Context) {
	requestBody, err := ioutil.ReadAll(conn.Request.Body)
	if err != nil {
		panic(err)
	}

	result := data.DeleteLibraryItem(requestBody)

	response, responseError := json.Marshal(result)
	if responseError != nil {
		log.Fatal(responseError)
	}
	conn.Data(http.StatusOK, "application/json", response)
}
