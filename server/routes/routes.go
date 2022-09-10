package routes

import (
	library "github.com/CampbellKT95/Reading/server/api"
	"github.com/CampbellKT95/Reading/server/tools"
	"github.com/gin-gonic/gin"
	cors "github.com/rs/cors/wrapper/gin"
)

func EstablishRoutes() {
	r := gin.Default()

	c := cors.New(cors.Options{
		AllowedOrigins:   []string{"*"},
		AllowCredentials: true,
		Debug:            true,
		AllowedMethods:   []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"},
	})
	r.Use(c)

	// library (readings)
	r.GET("/library", library.FetchLibrary)
	r.POST("/library", library.AddToLibrary)
	r.PUT("/library", library.UpdateLibraryItem)
	r.DELETE("/library", library.DeleteFromLibrary)

	// tools (translate word with Jisho API, etc)
	r.GET("/tools/translation/:word", tools.GetWordTranslation)
	r.GET("/tools/twitter/fetch", tools.FetchTweets)
	r.POST("/tools/twitter/add", tools.AddNewTweets)

	var port string = ":8080"
	r.Run(port)
}
