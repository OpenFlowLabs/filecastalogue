use clap::{App, Arg, SubCommand, 
    crate_authors, crate_description, crate_name, crate_version
};
use std::{env::current_dir, io};
use filecastalogue::{files::{blobs::drivers::local::LocalBlobFileCollection,
    indexes::{drivers::local::LocalIndexFileCollection},
    state::drivers::local::LocalStateFile}, finite_stream_handlers::LocalFile, journal::drivers::local::LocalJournal, repo::Repo};

const ABOUT_REPO: &str =
"Path to the repo directory. Defaults to the current directory.";
const ABOUT_VERSION: &str =
"Manage state versions.";
const ABOUT_ADD_VERSION: &str =
"Add a new version with the specified ID to the state.";

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
                    Repo::new(
                        LocalStateFile::new(LocalFile::new(repo_path))?,
                        LocalIndexFileCollection::new(),
                        LocalBlobFileCollection::new(),
                        LocalJournal::new()
                    );
                    Ok(())
                },
                None => Ok(())
            },
        None => Ok(())
        }
}