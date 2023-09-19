use candid::{CandidType, Deserialize};

pub type BankID = Vec<u8>;

#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Bank {
    id: BankID,
    name: String,
    main: bool,
}

impl Bank {
    pub fn new(id: BankID, name: String, main: bool) -> Self {
        Self { id, name, main }
    }

    pub fn id(&self) -> &BankID {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn main(&self) -> bool {
        self.main
    }

    pub fn set_main(&mut self, main: bool) {
        self.main = main;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
