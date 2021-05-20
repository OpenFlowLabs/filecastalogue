use crate::files::indexes::IndexFileCollection;

pub struct LocalIndexFileCollection {}

impl LocalIndexFileCollection {
    pub fn new() -> Self {
        Self {}
    }
}

impl IndexFileCollection for LocalIndexFileCollection {}