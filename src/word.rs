use crate::constants::RSM_WORD_SIZE;

#[derive(Clone)]
pub struct Word(String);

impl Word {
    pub fn new(value: String) -> Self {
        Word(value.chars().take(RSM_WORD_SIZE).collect())
    }
    pub fn empty() -> Self {
        Word("".into())
    }
}

impl ToString for Word {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<i32> for Word {
    fn from(value: i32) -> Self {
        Word(value.to_string())
    }
}

impl Into<String> for Word {
    fn into(self) -> String {
        self.0
    }
}

impl Into<Result<i32, String>> for Word {
    fn into(self) -> Result<i32, String> {
        self.0
            .parse()
            .map_err(|_| "Could not parse word into integer".into())
    }
}
