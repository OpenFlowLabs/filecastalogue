use std::env;
use std::process::exit;
use std::fs::copy;
use std::path::Path;
use std::process::Command;

enum HashAlgs {
    Blake3
}

/*
 * Steps through ever argument by position:
 *   - First argument: The source path of the file to hash.
 *   - Second argument: The target dir where the copy of the file (with the
 *     hash for its name) should go.
 *   - Third argument (optional): Hash algorithm.
 */
fn get_sane_args_or_exit(mut args: env::Args) -> (String, String, HashAlgs) {
    // Skip own name
    args.next();
    // First arg: Source path
    let source_path = match args.next() {
        None => {
            println!(concat!(
                "The first argument is missing: The path to the file you'd like to",
                "create a copy with a hashed version of the name of."
            ));
            // No path, empty string.
            "".to_string()
        },
        Some(x) => x
    };
    // Second arg: Target dir
    let target_dir_path= match args.next(){
        None => {
            println!(concat!(
                "The second argument is missing: The output dir where the copy with the ",
                "hashed name is supposed to go."
            ));
            // No path, empty string.
            "".to_string()
        },
        Some(x) => x
    };
    // Third arg: Algorithm
    let alg= match args.next() {
        None => {
            println!(concat!(
            "The third argument doesn't specify an algorithm. Choosing default: blake3."
            ));
            HashAlgs::Blake3
        },
        Some(x) => match x {
            _blake3 => HashAlgs::Blake3
        }
    };

    // Empty paths are either the result of sanity checks or something that isn't
    // quite right. In any case, we can't do anything with those.
    if source_path == "" || target_dir_path == "" {
        exit(1);
    }
    else {
        return (source_path, target_dir_path, alg);
    }
}

/* Prints the specified error message and exits with the specified exit code. */
fn errexit(msg: String, code: i32) -> ! {
    println!("Error: {}. Exiting.", msg);
    exit(code);
}

fn blake3_file_hash<'a>(source_path: &'a String) -> String {
    let hasher_cmd_result =
        match Command::new("b3sum")
            .arg(&source_path)
            .output() {
                Ok(x) => x,
                Err(e) => errexit(
                    format!("Whilst trying to hash the file at \"{}\", this error happened: {}", source_path, e.to_string()),
                    1
                )
            };
    let stdout = hasher_cmd_result.stdout;
    let stdout_ref = String::from_utf8(stdout);
    let stdout_str: String;
    match stdout_ref {
        Ok(x) => stdout_str = x,
        Err(e) => errexit(
            format!("There was a problem getting the string representation of b3sum's output. Original error: {}", e),
            1
        )
    };
    let hash = match stdout_str.split_whitespace().next() {
        Some(hash) => String::from(hash),
        None => errexit(
            format!("There was a problem splitting the hash out of b3sum's output, with said output being: {}", stdout_str),
            1
        )
    };
    return hash
}

fn main() {
    // let hash = blake3::hash(b"Test");
    // println!("{:?}", hash);
    let (source_path, target_dir_path, alg) = get_sane_args_or_exit(env::args());
    // let file_name = Path::new(&source_path).file_name().unwrap().as_bytes();
    // let file_name_hash = match alg {
    //     HashAlgs::Blake3 => blake3::hash(file_name)
    // };
    // let file_name_hash_bytes = file_name_hash.to_hex();

    let file_hash = match alg {
        HashAlgs::Blake3 => blake3_file_hash(&source_path)
    };
    let hashed_target_path = Path::new(&target_dir_path).join(Path::new(&file_hash));
    match copy(&source_path, &hashed_target_path) {
        Ok(_) => {},
        Err(e)=> errexit(
            format!(
                "Whilst trying to copy the file at \"{}\" to {:?}, this happened: {:?}. Exiting.",
                source_path,
                hashed_target_path,
                e
            ),
            1
        )
    };
}