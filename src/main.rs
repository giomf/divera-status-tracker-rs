use std::{fs, path::Path};

use clap::Parser;
mod fetcher;

const OUTPUT: &str = "output";
const ATTACHMENTS: &str = "attachments";

#[derive(Parser)]
#[command(version, about = "Divera Status Tracker", long_about = None)]
#[clap(propagate_version = true)]
enum Cli {
    Fetch(Fetch),
}

#[derive(Debug, Parser)]
struct Fetch {
    #[clap(long)]
    pub email: String,
    #[clap(long)]
    pub host: String,
    #[clap(long)]
    pub password: String,
    #[clap(long)]
    pub subject: String,
}

fn main() {
    let cli = Cli::parse();
    let attachments_folder = Path::new(OUTPUT).join(ATTACHMENTS);
    fs::create_dir_all(&attachments_folder).expect("Failed to create attachments folder");

    match cli {
        Cli::Fetch(cmd) => fetcher::fetch_subject_messages(
            cmd.host,
            cmd.email,
            cmd.password,
            cmd.subject,
            &attachments_folder,
        ),
    };
}
