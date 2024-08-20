use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct DictCsv {
    word: String,
    phonetic: String,
    definition: String,
    translation: String,
    pos: String,
    collins: String,
    oxford: String,
    tag: String,
    bnc: String,
    frq: String,
    exchange: String,
    detail: String,
    audio: String,
}

pub struct Dict {
    csv_dict: HashMap<String, DictCsv>,
}

impl Dict {
    pub fn new(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut csv_dict = HashMap::new();
        let mut reader = csv::Reader::from_path(file_path)?;

        for result in reader.deserialize() {
            let record: DictCsv = result?;
            csv_dict.entry(record.word.clone()).or_insert(record);
        }

        Ok(Dict { csv_dict })
    }

    pub fn search_translation(&self, word: &str) -> String {
        //println!("{}",self.csv_dict[word].translation.clone());
        if self.csv_dict.keys().any(|x| x == word) {
            self.csv_dict[word].translation.clone()
        } else {
            "not find".to_string()
        }
    }
}
