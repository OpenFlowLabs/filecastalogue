pub mod drivers {
    pub mod local;
}

// Journal that does nothing but act as if everything was okay.
// Might need an implementation at some point.
pub struct OptimisticDummyJournal {}

impl OptimisticDummyJournal {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait Journal {}

impl Journal for OptimisticDummyJournal {}