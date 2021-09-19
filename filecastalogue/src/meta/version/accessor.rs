use super::{
    // error::{},
    model::Version
};

pub trait VersionAccessor<'acc> {
    fn has_index(&self) -> bool;
    fn get_index_id(&self) -> Option<String>;
    fn set_index_id(&mut self, index_id: &str) -> String;
}

impl<'acc> VersionAccessor<'acc> for Version {
    
    /// Returns true if there's an index hash stored for this version.
    /// 
    /// Returns false otherwise.
    fn has_index(&self) -> bool {
        self.index != None
    }

    /// Returns the stored hash of the index for this version if it has one.
    /// 
    /// If there is no index hash stored for this version, this returns
    /// None.
    fn get_index_id(&self) -> Option<String> {
        if self.has_index() {
            self.index.to_owned()
        }
        else {
            None
        }
    }

    /// Sets the stored index hash for this version.
    /// 
    /// Returns the newly set index hash.
    fn set_index_id(&mut self, index_id: &str) -> String {
        self.index = Some(index_id.to_owned());
        index_id.to_owned()
    }
}