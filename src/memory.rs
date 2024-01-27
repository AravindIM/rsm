use crate::{constants::RSM_MEMORY_NUM_WORDS, errors::MemoryError, word::Word};

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

    pub fn get_word(&self, address: i32) -> Result<Word, MemoryError> {
        if address < 0 {
            return Err(MemoryError::IllegalAccess);
        }
        self.mem
            .get(address as usize)
            .ok_or(MemoryError::IllegalAccess)
            .map(|res| res.clone())
    }

    pub fn put_word(&mut self, address: i32, word: Word) -> Result<(), MemoryError> {
        if address < 0 {
            return Err(MemoryError::IllegalAccess);
        }
        match self.mem.get_mut(address as usize) {
            Some(elem) => {
                *elem = word;
                Ok(())
            }
            None => Err(MemoryError::IllegalAccess),
        }
    }
}
