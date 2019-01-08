mod onepass;

use std::path::PathBuf;
use structopt::StructOpt;
use crate::onepass::OnePassClient;

#[derive(StructOpt, Debug)]
#[structopt(name = "enigma")]
enum Opt {
    /// Save a secret.
    #[structopt(name = "save")]
    Save {
        #[structopt(subcommand)]
        subcommand: SaveSubcommand,
    },
}

#[derive(StructOpt, Debug)]
#[structopt()]
enum SaveSubcommand {
    #[structopt(name = "env")]
    /// Save an environment variable secret.
    Env {
        /// The name of the secret.
        name: String,
        /// The name of the environment variable. (ie. GITHUB_TOKEN)
        variable: String,
        /// The environment variable secret.
        secret: String,
    },
    #[structopt(name = "file")]
    /// Save a secret file.
    File {
        /// The name of the secret.
        name: String,
        /// The path(s) of the secret files to store.
        #[structopt(parse(from_os_str))]
        paths: Vec<PathBuf>,
    },
}

fn main() {
    let opt = Opt::from_args();

    match opt {
        Opt::Save { subcommand } => {
            match subcommand {
                SaveSubcommand::Env { name, variable, secret } => {
                    let client = OnePassClient::new(None).unwrap();
                    match client.set_variable(&name, &variable, &secret) {
                        Ok(_) => println!("saved {} to {}", name, variable),
                        Err(err) => eprintln!("could not save: {}", err),
                    };
                },
                SaveSubcommand::File { name, paths } => {
                    println!("will save {} at {:?}", name, paths);
                },
            }
        }
    };
}
