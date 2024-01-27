use crate::{
    constants::{RSM_MEMORY_NUM_WORDS, RSM_PAGE_SIZE},
    errors::MemoryError,
    word::Word,
};

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

    pub fn translate_page(
        &self,
        ptbr: i32,
        ptlr: i32,
        page: i32,
        is_write: bool,
    ) -> Result<i32, MemoryError> {
        let page_entry: i32;
        let page_info: i32;
        let entry: i32;
        let page_entry_w: Word;
        let page_info_w: Word;
        let info: String;

        if page < 0 || page >= ptlr {
            return Err(MemoryError::OutsideLogicalSpace);
        }

        page_entry = page * 2 + ptbr;
        page_info = page_entry + 1;

        page_entry_w = self.get_word(page_entry)?;
        page_info_w = self.get_word(page_info)?;

        entry = page_entry_w.into();
        info = page_info_w.into();

        if info.chars().nth(0).unwrap_or_default() == '0' {
            return Err(MemoryError::PageFault);
        }

        if is_write && info.chars().nth(0).unwrap_or_default() == '0' {
            return Err(MemoryError::AccessViolation);
        }

        Ok(entry)
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
