mod add;
mod branch;
mod checkout;
mod commit;
mod config;
mod diff;
mod helper;
mod init;
mod log;
mod status;
mod tag;

use add::add;
use branch::{branch, delete_branch};
use checkout::checkout;
use clap::{Arg, Command};
use commit::commit;
use config::{add_remote, handle_config_command, remove_remote};
use diff::diff;
use init::init;
use log::log;
use status::status;
use tag::{delete_tag, list_tags, tag};

fn main() {
    // CLI interface
    let matches =
        Command::new("rgit")
            .version("0.1.0")
            .about("Rgit is a Git implementation in Rust")
            .author("Kei-K23")
            .subcommand(
                Command::new("init")
                    .about("Create an empty Git repository or reinitialize an existing one"),
            )
            .subcommand(Command::new("log").about("Show commit log"))
            .subcommand(
                Command::new("add")
                    .about("Add file contents to the index")
                    .arg(Arg::new("file").required(true).help("The file to add")),
            )
            .subcommand(
                Command::new("checkout")
                    .about("Checkout to commit and switch branch")
                    .arg(
                        Arg::new("name")
                            .required(true)
                            .help("Branch name or commit key"),
                    ),
            )
            .subcommand(
                Command::new("branch")
                    .about("List, create, or delete branches")
                    .arg(Arg::new("name").required(false).help("Create new branch"))
                    .arg(
                        Arg::new("delete_branch")
                            .required(false)
                            .short('d')
                            .long("delete")
                            .help("Delete the branch"),
                    ),
            )
            .subcommand(
                Command::new("tag")
                    .about("Create, list, delete tags")
                    .arg(Arg::new("name").required(false).help("Create new tag"))
                    .arg(
                        Arg::new("delete_tag")
                            .required(false)
                            .short('d')
                            .long("delete")
                            .help("Delete the tag"),
                    ),
            )
            .subcommand(Command::new("status").about("Show the working tree status"))
            .subcommand(
                Command::new("diff")
                    .about("Show changes between commits, commit and working tree, etc"),
            )
            .subcommand(
                Command::new("commit")
                    .about("Record changes to the repository")
                    .arg(Arg::new("message").required(true).help("Commit message")),
            )
            .subcommand(
                Command::new("config")
                    .about("Configuration for repository")
                    .subcommand(
                        Command::new("set")
                            .about("Set configuration for the repository")
                            .arg(
                                Arg::new("key")
                                    .required(true)
                                    .help("The configuration key (e.g., 'name' or 'email')"),
                            )
                            .arg(Arg::new("value").required(true).help(
                                "The value to set (e.g., 'John Doe' or 'johndoe@example.com')",
                            )),
                    )
                    .subcommand(
                        Command::new("get")
                            .about("Get configuration of the repository")
                            .arg(Arg::new("key").required(true).help(
                                "The configuration key to retrieve (e.g., 'name' or 'email')",
                            )),
                    ),
            )
            .subcommand(
                Command::new("remote")
                    .about("Configuration for remote config")
                    .subcommand(
                        Command::new("add")
                            .about("Add remote configuration")
                            .arg(Arg::new("name").required(true).help("Remote config name"))
                            .arg(Arg::new("url").required(true).help("Remote config url")),
                    )
                    .subcommand(
                        Command::new("remove")
                            .about("Remove remote configuration of the repository")
                            .arg(
                                Arg::new("name")
                                    .required(true)
                                    .help("Remote config name to remove')"),
                            ),
                    ),
            )
            .get_matches();

    // Handle the init command
    if let Some(_) = matches.subcommand_matches("init") {
        if let Err(e) = init() {
            eprintln!("Error initializing repository: {}", e);
        }
    }
    // Handle the log command
    if let Some(_) = matches.subcommand_matches("log") {
        if let Err(e) = log() {
            eprintln!("Error when retrieve commit logs: {}", e);
        }
    }

    // Handle the status command
    if let Some(_) = matches.subcommand_matches("status") {
        if let Err(e) = status() {
            eprintln!("Error when retrieve the status of repository: {}", e);
        }
    }

    // Handle the diff command
    if let Some(_) = matches.subcommand_matches("diff") {
        if let Err(e) = diff() {
            eprintln!("Error when retrieve the changes of working tree: {}", e);
        }
    }

    // Handle the add command
    if let Some(add_matches) = matches.subcommand_matches("add") {
        if let Some(file_path) = add_matches.get_one::<String>("file") {
            if let Err(e) = add(file_path) {
                eprintln!("Error adding file to the staging area: {}", e);
            }
        }
    }

    // Handle the checkout command
    if let Some(checkout_matches) = matches.subcommand_matches("checkout") {
        if let Some(name) = checkout_matches.get_one::<String>("name") {
            if let Err(e) = checkout(name) {
                eprintln!("{}", e);
            }
        }
    }

    // Handle the branch command
    if let Some(branch_matches) = matches.subcommand_matches("branch") {
        let delete_branch_name = branch_matches.get_one::<String>("delete_branch");

        // Delete branch case
        if let Some(branch_name) = delete_branch_name {
            if let Err(e) = delete_branch(branch_name) {
                eprintln!("Error when deleting the branch: {}", e);
            }
        } else {
            // Create or list branch case
            let new_branch_name = branch_matches.get_one::<String>("name");

            if let Err(e) = branch(new_branch_name) {
                eprintln!("Error when creating new branch: {}", e);
            }
        }
    }

    // Handle the tag command
    if let Some(tag_matches) = matches.subcommand_matches("tag") {
        let delete_tag_name = tag_matches.get_one::<String>("delete_tag");
        let new_tag_name = tag_matches.get_one::<String>("name");

        if let Some(delete_tag_file) = delete_tag_name {
            // Delete branch case
            if let Err(e) = delete_tag(&delete_tag_file) {
                eprintln!("Error when deleting the tag: {}", e);
            }
        } else {
            if let Some(tag_name) = new_tag_name {
                if let Err(e) = tag(tag_name) {
                    eprintln!("Error when creating new tag: {}", e);
                }
            } else {
                if let Err(e) = list_tags() {
                    eprintln!("Error when retrieve list of tags: {}", e);
                }
            }
        }
    }

    // Handle the commit command
    if let Some(commit_matches) = matches.subcommand_matches("commit") {
        if let Some(message) = commit_matches.get_one::<String>("message") {
            if let Err(e) = commit(message) {
                eprintln!("Error committing file to the repository: {}", e);
            }
        }
    }

    // Handle the config set and get command
    if let Some(config_matches) = matches.subcommand_matches("config") {
        if let Some(set_matches) = config_matches.subcommand_matches("set") {
            let key = set_matches.get_one::<String>("key").unwrap();
            let value = set_matches.get_one::<String>("value").unwrap();
            if let Err(err) = handle_config_command("set", key, "[user]", Some(value)) {
                eprintln!("Error setting configuration: {}", err);
            }
        }

        if let Some(get_matches) = config_matches.subcommand_matches("get") {
            let key = get_matches.get_one::<String>("key").unwrap();
            if let Err(err) = handle_config_command("get", key, "[user]", None) {
                eprintln!("Error getting configuration: {}", err);
            }
        }
    }

    // Handle the remote config
    if let Some(remote_matches) = matches.subcommand_matches("remote") {
        if let Some(add_matches) = remote_matches.subcommand_matches("add") {
            let name = add_matches.get_one::<String>("name").unwrap();
            let url = add_matches.get_one::<String>("url").unwrap();
            if let Err(err) = add_remote(&name, &url) {
                eprintln!("Error adding remote configuration: {}", err);
            }
        }

        if let Some(remove_matches) = remote_matches.subcommand_matches("remove") {
            let name = remove_matches.get_one::<String>("name").unwrap();
            if let Err(err) = remove_remote(&name) {
                eprintln!("Error removing remote configuration: {}", err);
            }
        }
    }
}
