use clap::{App, Arg, SubCommand,
    crate_authors, crate_description, crate_name, crate_version
};
use std::{env::current_dir, ffi::OsString, io, path::PathBuf};
use filecastalogue::{
    error::Error, 
    files::{
        index_collection::MiscIndexFileCollection,
        state_collection::MiscStateFileCollection,
        tracked_ordinary_blob_collection::MiscTrackedOrdinaryBlobFileCollection
    },
    journal::OptimisticDummyJournal,
    opaque_collection_handlers::LocalDir,
    repo::Repo
};

const ABOUT_REPO: &str =
"Path to the repo directory. Defaults to the current directory.";
const ABOUT_VERSION: &str =
"Manage state versions.";
const ABOUT_ADD_VERSION: &str =
"Add a new version with the specified ID to the state.";

// Maybe push this into the library later, in some form.
fn create_local_repo
(repo_path: PathBuf)
-> Result<
    Repo<
        MiscStateFileCollection<LocalDir>,
        MiscIndexFileCollection<LocalDir>,
        MiscTrackedOrdinaryBlobFileCollection<LocalDir>,
        OptimisticDummyJournal
    >,
    Error
>
{
    let blob_dir_path = PathBuf::from(&repo_path).join(OsString::from("blobs"));
    // Indexes go into the same directory as blobs.
    let index_dir_path = PathBuf::from(&repo_path).join(OsString::from("blobs"));
    Ok(Repo::new(
        MiscStateFileCollection::new(LocalDir::new(&repo_path), OsString::from("state.json")),
        MiscIndexFileCollection::new(LocalDir::new(&index_dir_path)),
        // TODO [prio:critical]: repo_path is actually wrong here,
        // it's just there to test the typing atm.
        MiscTrackedOrdinaryBlobFileCollection::new(LocalDir::new(&blob_dir_path)),
        OptimisticDummyJournal::new()
    ))
}

fn main () -> Result<(), io::Error> {

    let default_repo_path  = current_dir().unwrap().as_os_str().to_owned();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(Arg::with_name("repo")
            .short("r")
            .long("repo")
            .help(ABOUT_REPO)
            .default_value_os(&default_repo_path)
            .takes_value(true))
        .subcommand(SubCommand::with_name("version")
            .about(ABOUT_VERSION)
            .subcommand(SubCommand::with_name("add"))
                .about(ABOUT_ADD_VERSION)
                .arg(Arg::with_name("id")))
        .get_matches();

    match matches.subcommand_matches("version") {
        Some(version_subcommand) =>
            match version_subcommand.subcommand_matches("add") {
                Some(version_id) => {
                    // The repo path is supposed to have a default. If this panics,
                    // that means we've run into a bug that made setting the default fail.
                    let repo_path = matches.value_of_os("repo").unwrap();
                    create_local_repo(PathBuf::from(repo_path));
                    Ok(())
                },
                None => Ok(())
            },
        None => Ok(())
        }
}