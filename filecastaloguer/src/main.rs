
//use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand, };
use clap::*;
use filecastalogue::{
    error::Error,
    files::{
        index_collection::MiscIndexFileCollection,
        state_collection::MiscStateFileCollection,
        tracked_ordinary_blob_collection::MiscTrackedOrdinaryBlobFileCollection
    },
    journal::OptimisticDummyJournal,
    opaque_collection_handler::drivers::local::LocalDir,
    repo::Repo,
};
use std::env::args;
use std::result::Result;
use std::{env::current_dir, ffi::OsString, io, path::PathBuf};

const ABOUT_REPO: &str = "Path to the repo directory. Defaults to the current directory.";
const ABOUT_VERSION: &str = "Manage state versions.";
const ABOUT_ADD_VERSION: &str = "Add a new version with the specified ID to the state.";

// impl Error::error for clap{

// }

// impl std::fmt::Display for Option<&str> {

// }

// Maybe push this into the library later, in some form.
/// Doc goes here, example, variants
fn create_local_repo(
    repo_path: PathBuf,
) -> Result<
    Repo<
        MiscStateFileCollection<LocalDir>,
        MiscIndexFileCollection<LocalDir>,
        MiscTrackedOrdinaryBlobFileCollection<LocalDir>,
        OptimisticDummyJournal
    >,
    Error,
> {
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

// #[derive(Display)];
// let default_repo_path = current_dir().unwrap().as_os_str().to_owned();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //test
    println!("{:?}", args());

    // #[derive(Display)];
    let default_repo_path = current_dir().unwrap().as_os_str().to_owned();

    //let default_path: PathBuf = PathBuf::new();

    //TODO [prio:candy]: cli autocompletion

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        // .setting(clap::AppSettings::TrailingVarArg)
        // .setting(clap::AppSettings::AllowLeadingHyphen)
        .arg(
            Arg::with_name("repo")
                .short("r")
                .long("repo")
                .help(ABOUT_REPO)
                .default_value_os(&default_repo_path)
                .takes_value(true),
        )
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
        .subcommand(
            SubCommand::with_name("new")
                // TODO [prio:v0.1]: Implement.
                .subcommand(
                    SubCommand::with_name("repository").arg(
                        Arg::with_name("path")
                            //.short("p")
                            .takes_value(true),
                    ),
                ),
                //start test area
                // .subcommand(SubCommand::with_name("test")
                //     .arg(Arg::with_name("path")
                //         .short("p")
                //         .takes_value(true)
                //     )
                //     .arg(Arg::with_name("test")
                //         .short("t")
                //     )
                // )
                //end test area
        )
        .subcommand(
            SubCommand::with_name("add")
                // TODO [prio:v0.1]: Implement.
                .subcommand(
                    SubCommand::with_name("version").arg(
                        //IMPORTANT major changes incoming with version stuff
                        Arg::with_name("version_id")
                            .short("i") //???
                            .takes_value(true), //.required(true) //???
                    ),
                )
                // TODO [prio:v0.1]: Implement.
                .subcommand(
                    SubCommand::with_name("file").arg(
                        Arg::with_name("path")
                            .short("p")
                            .takes_value(true)
                            .required(true),
                    ),
                )
                // TODO [prio:v0.1]: Implement.
                .subcommand(
                    SubCommand::with_name("directory").arg(
                        Arg::with_name("path")
                            //.short("p")
                            .takes_value(true)
                            .required(true),
                    ),
                ),
        )
        // TODO [prio:backlog]: Clarify and evaluate.
        .subcommand(
            SubCommand::with_name("update").subcommand(
                SubCommand::with_name("version").arg(
                    Arg::with_name("version_id")
                        //.short("id")
                        .takes_value(true)
                        .required(true), //.validator(String::from(""))
                ),
            ),
        )
        .subcommand(
            SubCommand::with_name("insert").subcommand(
                SubCommand::with_name("version")
                    // TODO [prio:v0.1]: Implement.
                    .subcommand(
                        SubCommand::with_name("before")
                            .arg(
                                Arg::with_name("version_id")
                                    .takes_value(true)
                                    .required(true),
                            )
                            .arg(
                                Arg::with_name("new_version_id")
                                    .takes_value(true)
                                    .required(true),
                            ),
                    )
                    // TODO [prio:v0.1]: Implement.
                    .subcommand(
                        SubCommand::with_name("after")
                            .arg(
                                Arg::with_name("version_id")
                                    .takes_value(true)
                                    .required(true),
                            )
                            .arg(
                                Arg::with_name("new_version_id")
                                    .takes_value(true)
                                    .required(true),
                            ),
                    ),
            ),
        )
        // TODO [prio:backlog]: Lay out requirements and implement.
        .subcommand(SubCommand::with_name("report"))
        .subcommand(
            SubCommand::with_name("remove")
                // TODO [prio:v0.1]: Implement.
                .subcommand(
                    SubCommand::with_name("version").arg(
                        Arg::with_name("version_id")
                            .takes_value(true)
                            .required(true),
                    ),
                ),
        )
        // TODO [prio:v0.1]: Implement.
        .subcommand(SubCommand::with_name("apply"))
        .subcommand(
            SubCommand::with_name("list")
                //.subcommand(SubCommand::with_name("versions"))
                // TODO [prio:v0.1]: Clarify what exactly each of them
                // is supposed to do and implement accordingly.
                .subcommands(vec![
                    // [aziroshin]: Maybe this could show just the versions?
                    SubCommand::with_name("versions"),
                    // [aziroshin]: Whilst this could show the files grouped
                    //              by versions? In that case, it could also
                    //              just be made a flag for `list`, though.
                    SubCommand::with_name("files"),
                ]),
        )
        // TODO [prio:backlog]: Implement.
        .subcommand(
            SubCommand::with_name("dublicate").subcommand(
                SubCommand::with_name("version")
                    .arg(
                        Arg::with_name("version_id")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("new_version_id")
                            .takes_value(true)
                            .required(true),
                    ),
            ),
        )
        .subcommand(
            SubCommand::with_name("delete")
            // TODO [prio:backlog]: Implement.
            //  The "backlog" prio reflects the notion that for v0.1,
            //  it might be acceptable to just have to delete the
            //  repo dir by hand.
            .subcommand(
                SubCommand::with_name("repository").arg(
                    Arg::with_name("path")
                        //.short("p")
                        .takes_value(true),
                ),
            ),
        )
        //.takes_value(false)
        // .subcommand(SubCommand::with_name("files"))
        // .subcommand(SubCommand::with_name("versions"))
        .get_matches();

    //println!("{:?}", matches);
    println!("{:#?}", matches);

    if let Some(matches) = matches.subcommand_matches("list") {
        if matches.is_present("files") {
            println!("list files");
        } else if matches.is_present("versions") {
            println!("list versions");
        }
    }

    // if let Some(matches) is needed except for the last "branch"
    if let Some(matches) = matches.subcommand_matches("new") {
        if let Some(matches) = matches.subcommand_matches("repository") {
            if matches.is_present("path") {
                //value_of("path") should be validated somewhere
                let path = matches.value_of("path").unwrap();
                create_local_repo(PathBuf::from(path))?;

                println!("create new repository in {} ", path);
            } else {
                create_local_repo(PathBuf::from(&default_repo_path))?;
                println!("create new repository in {:?} ", default_repo_path);
            }
        }
        //start test area
        // else if let Some(matches) = matches.subcommand_matches("test"){
        //     if matches.is_present("path"){
        //         if matches.is_present("test"){
        //             println!("test!!")
        //         }
        //         println!("create new repository in {:#?} ", matches.value_of("path"));
        //     } else {
        //         if matches.is_present("test"){
        //             println!("test!!")
        //         }
        //         println!("create new repository in {:#?} ", default_repo_path);
        //     }
        // }
        //end test area
    }

    if let Some(matches) = matches.subcommand_matches("add") {
        if let Some(matches) = matches.subcommand_matches("version") {
            if matches.is_present("version_id") {
                //let version_id = matches.value_of("version_id").unwrap();
                println!(
                    "add new version with id: {:?} ",
                    matches.value_of("version_id")
                );
            } else {
                println!("add new version with id: {:?} ", "default_version_id");
            }
        }
        if let Some(matches) = matches.subcommand_matches("file") {
            if matches.is_present("version_id") {
                let path = matches.value_of("version_id").unwrap();
                println!(
                    "add new version with id: {:#?} ",
                    matches.value_of("version_id")
                );
            } else {
                println!("add new version with id: {:?} ", "default_version_id");
            }
        }
        if let Some(matches) = matches.subcommand_matches("directory") {
            if matches.is_present("version_id") {
                let path = matches.value_of("version_id").unwrap();
                println!(
                    "add new version with id: {:#?} ",
                    matches.value_of("version_id")
                );
            } else {
                println!("add new version with id: {:?} ", "default_version_id");
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("insert") {
        if let Some(matches) = matches.subcommand_matches("version") {
            if let Some(matches) = matches.subcommand_matches("before") {
                if matches.is_present("version_id") {
                    let version_id = matches.value_of("version_id").unwrap();
                    if matches.is_present("new_version_id") {
                        let new_version_id = matches.value_of("new_version_id").unwrap();
                        println!(
                            "insert new version before version {} with id: {}",
                            version_id, new_version_id,
                        )
                    }
                }
            }
            if let Some(matches) = matches.subcommand_matches("after") {
                if matches.is_present("version_id") {
                    let version_id = matches.value_of("version_id").unwrap();
                    if matches.is_present("new_version_id") {
                        let new_version_id = matches.value_of("new_version_id").unwrap();
                        println!(
                            "insert new version before version {} with id: {}",
                            version_id, new_version_id,
                        )
                    }
                }
            }
        }
    }


    if let Some(matches) = matches.subcommand_matches("update") {
        if let Some(matches) = matches.subcommand_matches("version") {
            if matches.is_present("version_id") {
                //value_of("path") should be validated somewhere
                let version_id = matches.value_of("version_id").unwrap();
                println!("updated version {} ", version_id);
            } else {
                
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("remove") {
        if let Some(matches) = matches.subcommand_matches("version") {
            if matches.is_present("version_id") {
                //value_of("path") should be validated somewhere
                let version_id = matches.value_of("version_id").unwrap();
                println!("removed version {} ", version_id);
            } else {
                
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("delete") {
        if let Some(matches) = matches.subcommand_matches("repository") {
            if matches.is_present("path") {
                //value_of("path") should be validated somewhere
                let path = matches.value_of("path").unwrap();
                println!("deleted repository in {} ", path);
            } else {
                println!("deleted repository in {:?} ", default_repo_path);
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("apply") {
        println!("apply data.")
    }

    match matches.subcommand() {
        _ => {}
    }

    match matches.value_of("test").unwrap_or_default() {
        "none" => println!("none"),
        "testos" => println!("testos"),
        _ => (),
    }

    //println!("test");

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

    // match matches.subcommand_matches("version"){
    //     Some(version_subcommand) => match version_subcommand.subcommand_matches("add"){
    //         none => (),
    //         version_id =>{
    //             let repo_path = matches.value_of_os("repo").unwrap(); //+default
    //             create_local_repo(PathBuf::from(repo_path));
    //             println!("repo added");
    //         } ,
    //         _ => ()
    //     }
    //     _ => ()
    // }

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

/// this is the rustdoc testfunction.
/// rustdoc does only generates documentation for public functions per default
pub fn testdoc() -> String {
    /// return a empty string
    String::from("")
}
