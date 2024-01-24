use crate::{constants::RSM_MEMORY_NUM_WORDS, word::Word};

pub struct Memory {
    mem: Vec<Word>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            mem: std::iter::from_fn(move || Some(Word::empty()))
                .take(RSM_MEMORY_NUM_WORDS)
                .collect(),
        }
    }

    pub fn get_word(&self, index: usize) -> Result<Word, String> {
        self.mem
            .get(index)
            .ok_or(format!("Cannot access memory at {index}"))
            .map(|res| res.clone())
    }
}
