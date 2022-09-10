package models

type JishoApi struct {
	Meta JishoMeta
	Data []JishoData
}

type JishoMeta struct {
	Status int
}

type JishoData struct {
	Slug        string
	Is_common   bool
	Jlpt        []string
	Japanese    []JishoJapanese
	Senses      []JishoSenses
	Attribution JishoAttribution
}

type JishoJapanese struct {
	Word    string
	Reading string
}

type JishoSenses struct {
	English_definition []string
	Parts_of_speech    []string
	// there are many more parts to this, but have left out since it is unneeded
}

// the values below actually vary between bool & string
type JishoAttribution struct {
	Jmidct   bool
	Jmnedict bool
	Dbpedia  string
}
