
use std::fs::{write, read_to_string};
use std::path::Path;
use std::io::Error;
mod index_json;
mod state_json;
mod state;

// Reference: https://docs.serde.rs/serde_json

fn main() -> Result<(), Error> {
    let state_path = "./filecastalogue_mess/state.json";
    let version_index_path = "./filecastalogue_mess/index.json";
    let state_target_dir = "./filecastalogue_mess/.tmp/state";
    let state_target_path = Path::new(state_target_dir).join("state.json");
    let version_index_target_path = Path::new(state_target_dir).join("index.json");

    // Read state file.
    let state_file_contents = read_to_string(state_path)?;
    // Get state struct from state file JSON.
    let state: state_json::State = serde_json::from_str(&state_file_contents)?;
    // Read index file.
    let json_input = read_to_string(version_index_path)?;
    // Get index struct from index file JSON.
    let index: index_json::Index = serde_json::from_str(&json_input)?;

    let nginx_aspects = &index.files["/etc/nginx/nginx.conf"];
    println!("{:?}", nginx_aspects);
    let nginx_aspects_details = match nginx_aspects {
        index_json::FileAspects::File(aspects) => aspects,
        _ => panic!("Not a FileFileKind")
    };
    
    // Test reads, so we can see this is doing anything sensible at all.
    println!("\nnginx entry aspects:");
    println!("{:?}", nginx_aspects_details);
    println!("\nstate.json:");
    println!("{:?}", state);

    // Test writes
    write(state_target_path, b"TEST for state")?;
    write(version_index_target_path, b"TEST for index")?;

    Ok(())
}