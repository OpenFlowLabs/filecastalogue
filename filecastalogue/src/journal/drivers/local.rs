use crate::journal::Journal;

pub struct LocalJournal {}

impl LocalJournal {
    pub fn new() -> Self {
        Self {}
    }
}

impl Journal for LocalJournal {}