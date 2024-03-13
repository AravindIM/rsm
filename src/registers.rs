use indexmap::IndexMap;

use crate::{
    constants::{REG_KERN_HIGH, REG_KERN_LOW, REG_PORT_HIGH, REG_PORT_LOW},
    word::Word,
};
pub struct Registers {
    registers: IndexMap<String, Word>,
}

impl Registers {
    pub fn new() -> Self {
        let reg_names = vec![
            "R0", "R1", "R2", "R3", "R4", "R5", "R6", "R7", "R8", "R9", "R10", "R11", "R12", "R13",
            "R14", "R15", "R16", "R17", "R18", "R19", "P0", "P1", "P2", "P3", "BP_REG", "SP_REG",
            "IP_REG", "PTBR_REG", "PTLR_REG", "EIP_REG", "EC_REG", "EPN_REG", "EMA_REG",
        ];
        let mut registers: IndexMap<String, Word> = IndexMap::new();
        for name in reg_names {
            registers.insert(String::from(name), Word::empty());
        }

        Registers { registers }
    }

    pub fn get(&self, name: &str) -> Result<Word, String> {
        match self.registers.get(name) {
            Some(w) => Ok(w.clone()),
            None => Err(format!("Invalid register {name} was accessed")),
        }
    }

    pub fn set(&mut self, name: &str, value: Word) -> Result<(), String> {
        self.get(name)?;
        self.registers.insert(String::from(name), value);
        Ok(())
    }

    pub fn is_umode(&self, name: String) -> bool {
        match self.registers.get_index_of(&name) {
            Some(index) => {
                (index < REG_PORT_LOW || index > REG_PORT_HIGH)
                    && (index < REG_KERN_LOW || index > REG_KERN_HIGH)
            }
            None => false,
        }
    }
}
