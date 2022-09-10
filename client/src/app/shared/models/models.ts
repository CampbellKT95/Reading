export interface Readings {
    Text: string,
    Translation: string
} 

export interface JishoReturn {
    Data: WordDescription[],
    Meta: JishoMeta
}

export interface JishoMeta {
    Status: 200
}

export interface WordDescription {
    Japanese: JishoJapanese[]
    Is_common: boolean,
    Jlpt: string[],
    Senses: JishoSenses[]
}

export interface JishoJapanese {
    Word: string,
    Reading: string
}

export interface JishoSenses {
    English_definition: string,
    Parts_of_speech: string[]
}

export interface Tweet {
    Id: string,
    Text: string
}