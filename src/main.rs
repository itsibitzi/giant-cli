mod credentials_store;
mod filesystem;
mod giant_api;

use clap::Parser;
use credentials_store::CredentialsStore;
use std::{error::Error, mem};

#[derive(Parser)]
#[clap(name = "giant")]
#[clap(bin_name = "giant")]
enum Giant {
    Login(Login),
    Mount(Mount),
}

#[derive(clap::Args)]
#[clap(author, version, about, long_about = None)]
struct Login {
    uri: String,
    token: String,
}

#[derive(clap::Args)]
#[clap(author, version, about, long_about = None)]
struct Mount {
    ingestion: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // This is a bit of a hack so we can map inodes to positions in a vector directly.
    // Realistically speaking we should never run into an issue where we'd have more than 2^32 entries
    // in a single Giant ingestion, but whatever...
    if mem::size_of::<usize>() != 8 {
        panic!("Not running on a 64 bit OS")
    }

    let mut creds_path = dirs::home_dir().unwrap_or("./".into());
    creds_path.push(".giant-creds");

    let mut creds = CredentialsStore::from_file(creds_path);

    match Giant::parse() {
        Giant::Login(Login { uri, token }) => {
            println!("Login to {} using {}", uri, token);
        }
        Giant::Mount(Mount { ingestion }) => println!("Mount {}", ingestion),
    }

    // Fetch all giant File nodes from manifest, constructing the inode ->
    Ok(())
}
