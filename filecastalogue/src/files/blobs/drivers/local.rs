use crate::files::blobs::BlobFileCollection;

pub struct LocalBlobFileCollection {}

impl LocalBlobFileCollection {
    pub fn new() -> Self {
        Self {}
    }
}

impl BlobFileCollection for LocalBlobFileCollection {}