use std::{collections::HashMap, io::Read};
use std::fs::File;
use serde::{Serialize, Deserialize};
// use serde_json::{Result, Value};

// Reference: https://docs.serde.rs/serde_json

#[derive(Serialize, Deserialize, Debug)]
struct Attributes {
    posix_user: String,
    posix_group: String
}

#[derive(Serialize, Deserialize, Debug)]
struct NonExistingFileKind {}

#[derive(Serialize, Deserialize, Debug)]
struct DirectoryFileKind {
    attributes: Attributes
}

#[derive(Serialize, Deserialize, Debug)]
struct FileFileKind {
    hash: String,
    attributes: Attributes
}

#[derive(Serialize, Deserialize, Debug)]
struct SymlinkFileKind {
    linked_to: String
}

// struct HardlinkFileKind {

// }

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "kind",
    rename_all(
        serialize = "snake_case",
        deserialize = "snake_case"
    )
)]

enum FileAspects {
    NonExisting(NonExistingFileKind),
    Directory(DirectoryFileKind),
    File(FileFileKind),
    Symlink(SymlinkFileKind)
}

#[derive(Serialize, Deserialize, Debug)]
struct VersionIndex {
    files: HashMap<String, FileAspects>
}

#[derive(Serialize, Deserialize, Debug)]
struct StateVersionEntry {
    index: String
}

#[derive(Serialize, Deserialize, Debug)]
struct State {
    versions: HashMap<String, StateVersionEntry>
}

fn main() {
    // Open state file.
    let mut state_file = match File::open("./filecastalogue_mess/state.json") {
        Ok(file) => file,
        Err(e) => panic!("{:?}", e)
    };
    // Read state file.
    let mut state_file_contents = String::new();
    match state_file.read_to_string(&mut state_file_contents) {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e)
    };
    // Get state struct from state file JSON.
    let state: State = match serde_json::from_str(&state_file_contents) {
        Ok(x) => x,
        Err(e) => panic!("{:?}", e)
    };
    // Open index file.
    let mut version_file = match File::open("./filecastalogue_mess/index.json") {
        Ok(file) => file,
        Err(e) => panic!("{:?}", e)
    };
    // Read index file.
    let mut json_input = String::new();
    match version_file.read_to_string(&mut json_input) {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e)
    };
    // Get index struct from index file JSON.
    let index: VersionIndex = match serde_json::from_str(&json_input) {
        Ok(x) => x,
        Err(e) => panic!("{:?}", e)
        
    };
    // println!("{:?}", index);
    let nginx_aspects = &index.files["/etc/nginx/nginx.conf"];
    println!("{:?}", nginx_aspects);
    let nginx_aspects_details = match nginx_aspects {
        FileAspects::File(aspects) => aspects,
        _ => panic!("Not a FileFileKind")
    };
    
    println!("\nnginx entry aspects:");
    println!("{:?}", nginx_aspects_details);
    println!("\nstate.json:");
    println!("{:?}", state);
}