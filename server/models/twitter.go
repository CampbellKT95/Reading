package models

type UserLookUpId struct {
	Data []UserLookUpData
}

type UserLookUpData struct {
	Id       string
	Name     string
	Username string
}

type TwitterPostLookup struct {
	Data []TwitterPostData
	Meta TwitterPostMeta
}

type TwitterPostData struct {
	Id   string
	Text string
}

type TwitterPostMeta struct {
	Oldest_id    string
	Newest_id    string
	Result_count int
	Next_token   string
}
