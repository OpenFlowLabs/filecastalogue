use serde::{Serialize, Deserialize};

/* Notes:
     This uses a flat structure, with a prefix_ namespacing
     attributes that somehow belong together, such as posix_.
     The idea is to not overcomplicate the JSON interface, avoid hard coding
     too many assumptions about the future and leave the door open
     for unusual combinations of attributes without having to foresee
     their use cases, whilst still retaining a reasonable degree of forward
     compatibility.
*/
/// Attributes like file permissions, groups or sticky bit.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Attributes {
    pub posix_user: String,
    pub posix_group: String
}
