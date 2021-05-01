use std::collections::HashMap;

use serde::{Serialize, Deserialize};
// use serde_json::{Result, Value};

#[derive(Serialize, Deserialize, Debug)]
struct Attributes {
    posix_user: String,
    posix_group: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
enum NonExistingFileKind {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
enum DirectoryFileKind {
    Attributes(Attributes)
}

#[derive(Serialize, Deserialize, Debug)]
struct FileFileKind {
    hash: String,
    attributes: Attributes
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
enum SymlinkFileKind {
    LinkedTo(String)
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
// Reference: https://docs.serde.rs/serde_json

#[derive(Serialize, Deserialize, Debug)]
struct Index {
    files: HashMap<String, FileAspects>
}

fn main() {
    let json_input =
r#"{
    "files": {
        "/etc/nginx/nginx.conf": {
            "kind": "file",
            "hash": "",
            "attributes": {
                "posix_user": "",
                "posix_group": ""
            }
        }
    }
}"#;
    let index: Index = match serde_json::from_str(json_input) {
        Ok(x) => x,
        Err(e) => {
            println!("{:?}", e);
            panic!(e);
        }
    };
    // println!("{:?}", index);
    let nginx_aspects = &index.files["/etc/nginx/nginx.conf"];
    println!("{:?}", nginx_aspects);
}