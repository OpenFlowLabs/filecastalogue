//use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand, };
use clap::*;
use filecastalogue::{
    error::Error,
    files::{
        blobs::drivers::local::LocalBlobFileCollection,
        indexes::drivers::local::LocalIndexFileCollection, state::drivers::local::StateFile,
    },
    finite_stream_handlers::LocalFile,
    journal::OptimisticDummyJournal,
    opaque_collection_handlers::LocalDir,
    repo::Repo,
};
use std::env::args;
use std::{env::current_dir, ffi::OsString, io, path::PathBuf};
use std::result::Result;

const ABOUT_REPO: &str = "Path to the repo directory. Defaults to the current directory.";
const ABOUT_VERSION: &str = "Manage state versions.";
const ABOUT_ADD_VERSION: &str = "Add a new version with the specified ID to the state.";

// impl Error::error for clap{

// } 
// Maybe push this into the library later, in some form.
fn create_local_repo(
    repo_path: PathBuf,
) -> Result<
    Repo<
        StateFile<LocalFile>,
        LocalIndexFileCollection<LocalDir>,
        LocalBlobFileCollection<LocalDir>,
        OptimisticDummyJournal,
    >,
    Error,
> {
    let blob_dir_path = PathBuf::from(&repo_path).join(OsString::from("blobs"));
    // Indexes go into the same directory as blobs.
    let index_dir_path = PathBuf::from(&repo_path).join(OsString::from("blobs"));
    let state_file_path = PathBuf::from(&repo_path).join(OsString::from("state.json"));
    let state_file = StateFile::new(LocalFile::new(state_file_path))?;
    Ok(Repo::new(
        state_file,
        LocalIndexFileCollection::new(LocalDir::new(&index_dir_path)),
        // TODO [prio:critical]: repo_path is actually wrong here,
        // it's just there to test the typing atm.
        LocalBlobFileCollection::new(LocalDir::new(&blob_dir_path)),
        OptimisticDummyJournal::new(),
    ))
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    //test
    println!("{:?}", args());

    let default_repo_path = current_dir().unwrap().as_os_str().to_owned();

    //let default_path: PathBuf = PathBuf::new();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        // .setting(clap::AppSettings::TrailingVarArg)
        // .setting(clap::AppSettings::AllowLeadingHyphen)
        .arg(Arg::with_name("repo")
            .short("r")
            .long("repo")
            .help(ABOUT_REPO)
            .default_value_os(&default_repo_path)
            .takes_value(true))
        // .subcommand(SubCommand::with_name("version")
        //     .about(ABOUT_VERSION)
        //     .subcommand(SubCommand::with_name("add"))
        //         .about(ABOUT_ADD_VERSION)
        //         .arg(Arg::with_name("id")))
        // .arg(
        //     Arg::with_name("init")
        //         //.short("r")
        //         //.long("init")
        //         .help(ABOUT_REPO)
        //         //.value_name("PATH")
        //         //.index(1)
        //         //.default_value_os(&default_repo_path)
        //         .takes_value(true),
        // )
        // .subcommand(
        //     SubCommand::with_name("version")
        //         .about(ABOUT_VERSION)
        //         .subcommand(SubCommand::with_name("add"))
        //         .about(ABOUT_ADD_VERSION)
        //         .arg(Arg::with_name("id")),
        // )
        .subcommand(SubCommand::with_name("new")
            .subcommand(SubCommand::with_name("repository")  
                .arg(Arg::with_name("path")
                    .short("p")
                    .takes_value(true)
                )
            )
            .subcommand(SubCommand::with_name("test")
                .arg(Arg::with_name("path")
                    .short("p")
                    .takes_value(true)
                )
                .arg(Arg::with_name("test")
                .short("t")
                )
            )
        )
        .subcommand(SubCommand::with_name("list")
            //.subcommand(SubCommand::with_name("versions"))
            .subcommands(vec![
                SubCommand::with_name("versions"),
                SubCommand::with_name("files"),
            ])
        )
            //.takes_value(false)
            // .subcommand(SubCommand::with_name("files"))
            
            // .subcommand(SubCommand::with_name("versions"))
    .get_matches();

    //println!("{:?}", matches);
    println!("{:#?}", matches);

    if let Some(matches) = matches.subcommand_matches("list") {
        if matches.is_present("files"){
            println!("list files");
        } else if matches.is_present("versions") {
            println!("list versions");
        }
    }

    if let Some(matches) = matches.subcommand_matches("new") {
        if let Some(matches) = matches.subcommand_matches("repository"){
            if matches.is_present("path"){
                //value_of("path") should be validated somewhere
                let path = matches.value_of("path").unwrap();
                create_local_repo(PathBuf::from(path));
                println!("create new repository in {:#?} ", matches.value_of("path"));
            } else {
                create_local_repo(PathBuf::from(&default_repo_path));
                println!("create new repository in {:?} ", default_repo_path);
            }
        } else if let Some(matches) = matches.subcommand_matches("test"){ //start test area
            if matches.is_present("path"){
                if matches.is_present("test"){
                    println!("test!!")
                }
                println!("create new repository in {:#?} ", matches.value_of("path"));
            } else {
                if matches.is_present("test"){
                    println!("test!!")
                }

                println!("create new repository in {:#?} ", default_repo_path);
            }
        } //end test area
    } 

    match matches.subcommand(){
        _ => {}
    }

    match matches.value_of("test").unwrap_or_default(){
        "none" => println!("none"),
        "testos" => println!("testos"),
        _ => ()
    }

    // match matches.subcommand_matches("version") {
    //     Some(version_subcommand) => match version_subcommand.subcommand_matches("add") {
    //         Some(version_id) => {
    //             // The repo path is supposed to have a default. If this panics,
    //             // that means we've run into a bug that made setting the default fail.
    //             let repo_path = matches.value_of_os("repo").unwrap();
    //             create_local_repo(PathBuf::from(repo_path));
    //             Ok(())
    //         }
    //         None => Ok(()),
    //     },
    //     None => Ok(()),
    // }

    match matches.subcommand_matches("version"){
        Some(version_subcommand) => match version_subcommand.subcommand_matches("add"){
            none => (),
            version_id =>{
                let repo_path = matches.value_of_os("repo").unwrap(); //+default
                create_local_repo(PathBuf::from(repo_path));
                println!("repo added");
            } ,
            _ => ()
        } 
        _ => ()
    } 

    // if matches.is_present("init"){
    //     println!("test {:?}", matches);//.value_of("init"));
    // };

    // let path = matches.value_of("init").unwrap_or_default();
    // println!("init {:?}", path);

    // match path {
    //     Ok(path) => {
    //         None => !println!(""),
    //         Some(path) => println!("{}", path),
    //     },
    //     Err(err) => !eprintln!("{}", err),
    // }

    Ok(())
}
