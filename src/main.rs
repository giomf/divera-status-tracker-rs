use clap::Parser;
use std::{fs, path::Path};

mod data;
mod fetcher;
mod io;

const OUTPUT: &str = "output";
const ATTACHMENTS: &str = "attachments";

#[derive(Parser)]
#[command(version, about = "Divera Status Tracker", long_about = None)]
#[clap(propagate_version = true)]
enum Cli {
    /// Fetch all attachments and aggregate them
    Fetch(Fetch),
    /// Aggregate attachments
    Aggregate(Aggregate),
    /// Create on-duty table
    OnDuty(OnDuty),
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
    #[clap(long)]
    pub off_duty_keyword: String,
}

#[derive(Debug, Parser)]
struct Aggregate {
    #[clap(long)]
    pub off_duty_keyword: String,
}

#[derive(Debug, Parser)]
struct OnDuty {}

fn main() {
    let cli = Cli::parse();
    let output_folder = Path::new(OUTPUT);
    let attachments_folder = output_folder.join(ATTACHMENTS);
    fs::create_dir_all(&attachments_folder).expect("Failed to create attachments folder");

    match cli {
        Cli::Fetch(cmd) => {
            fetcher::fetch_subject_messages(
                cmd.host,
                cmd.email,
                cmd.password,
                cmd.subject,
                &attachments_folder,
            );
            let df = data::aggregate_attachments(&attachments_folder, &cmd.off_duty_keyword);
            io::write_parquet(df, &output_folder);
        }
        Cli::Aggregate(cmd) => {
            let df = data::aggregate_attachments(&attachments_folder, &cmd.off_duty_keyword);
            io::write_parquet(df, &output_folder);
        }

        Cli::OnDuty(_) => {}
    };
}
