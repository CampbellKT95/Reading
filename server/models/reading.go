package models

type Reading struct {
	Text        string
	Translation string
}

type UpdateRequest struct {
	Original Reading
	Update   Reading
}
