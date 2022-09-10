package tools

import (
	"encoding/json"
	"io"
	"net/http"
	"strings"

	models "github.com/CampbellKT95/Reading/server/models"

	"github.com/gin-gonic/gin"
)

func GetWordTranslation(conn *gin.Context) {
	// word will come in the url
	uri := conn.Request.RequestURI

	wordToTranslate := strings.Split(uri, "/")[3]

	response, err := http.Get("https://jisho.org/api/v1/search/words?keyword=" + wordToTranslate)
	if err != nil {
		panic(err)
	}
	defer response.Body.Close()

	body, err := io.ReadAll(response.Body)
	if err != nil {
		panic(err)
	}

	// body is a complex, deeply nested object. need to unmarshal the data into a made interface
	var wordContent models.JishoApi
	parseError := json.Unmarshal(body, &wordContent)
	if err != nil {
		panic(parseError)
	}

	parsedWordContent, err := json.Marshal(wordContent)
	if err != nil {
		panic(err)
	}

	conn.Data(http.StatusOK, "application/json", parsedWordContent)
}
