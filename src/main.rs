mod onepass;

use std::path::PathBuf;
use std::env;
use structopt::StructOpt;
use crate::onepass::OnePassClient;

#[derive(StructOpt, Debug)]
#[structopt(name = "enigma")]
enum Opt {
    /// Save a secret
    #[structopt(name = "save")]
    Save {
        #[structopt(subcommand)]
        subcommand: SaveSubcommand,
    },
    /// Get a secret
    #[structopt(name = "get")]
    Get {
        #[structopt(subcommand)]
        subcommand: GetSubcommand,
    },
}

#[derive(StructOpt, Debug)]
#[structopt()]
enum SaveSubcommand {
    #[structopt(name = "env")]
    /// Save an environment variable secret
    Env {
        /// The name of the secret
        name: String,
        /// The name of the environment variable (ie. GITHUB_TOKEN)
        variable: String,
        /// The value of the secret. If unset, will use the value from the
        /// current environment
        secret: Option<String>,
    },
    #[structopt(name = "file")]
    /// Save a secret file
    File {
        /// The name of the secret
        name: String,
        /// The path(s) of the secret files to store
        #[structopt(parse(from_os_str))]
        paths: Vec<PathBuf>,
    },
}

#[derive(StructOpt, Debug)]
#[structopt()]
enum GetSubcommand {
    #[structopt(name = "env")]
    /// Get an environment variable secret
    Env {
        /// The name of the secret
        name: String,
        /// If set, enigma will print an export command for the environment variable.
        /// You can use it like so: `eval $(enigma get github-token)`
        #[structopt(short = "e", long = "export")]
        export: bool,
    },
    #[structopt(name = "file")]
    /// Get a secret file
    File {
        /// The name of the secret
        name: String,
        /// The path to extract the secret
        #[structopt(parse(from_os_str))]
        paths: PathBuf,
    },
}

fn main() {
    let opt = Opt::from_args();

    match opt {
        Opt::Save { subcommand } => {
            match subcommand {
                SaveSubcommand::Env { name, variable, secret } => {
                    let value = if let Some(value) = secret {
                        value
                    } else {
                        match env::var(&variable) {
                            Ok(value) => value,
                            Err(e) => {
                                eprintln!("could not find {} in current environment: {}", &variable, e);
                                return;
                            },
                        }
                    };

                    let client = OnePassClient::new(None).unwrap();
                    match client.set_variable(&name, &variable, &value) {
                        Ok(_) => println!("saved {} to {}", name, variable),
                        Err(err) => eprintln!("could not save: {}", err),
                    };
                },
                SaveSubcommand::File { name, paths } => {
                    println!("will save {} at {:?}", name, paths);
                },
            }
        },
        Opt::Get { subcommand } => {
            match subcommand {
                GetSubcommand::Env { name, export } => {
                    let client = OnePassClient::new(None).unwrap();
                    match client.get_variable(&name) {
                        Some((variable, value)) => {
                            let export_command = if export {
                                "export "
                            } else {
                                ""
                            };

                            println!("{}{}={}", export_command, variable, value);
                        },
                        None => eprintln!("Secret '{}' not found", &name),
                    }
                },
                _ => unimplemented!(),
            }
        },
    };
}
