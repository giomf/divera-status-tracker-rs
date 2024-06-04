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
    /// Fetch all attachments
    Fetch(Fetch),
    /// Aggregate attachments
    Aggregate(Aggregate),
    /// Fetch all attachments and aggregate them
    Update(Update),
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
}

#[derive(Debug, Parser)]
struct Aggregate {
    #[clap(long)]
    pub off_duty_keyword: String,
}

#[derive(Debug, Parser)]
struct Update {
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
struct OnDuty {
    #[clap(long)]
    pub print: bool,

    #[clap(long)]
    pub export: bool,
}

fn main() {
    let cli = Cli::parse();
    let output_folder = Path::new(OUTPUT);
    let attachments_path = output_folder.join(ATTACHMENTS);
    let aggregation_path = output_folder.join("aggregation.parquet");
    fs::create_dir_all(&attachments_path).expect("Failed to create attachments folder");

    match cli {
        Cli::Fetch(cmd) => {
            fetcher::fetch_subject_messages(
                cmd.host,
                cmd.email,
                cmd.password,
                cmd.subject,
                &attachments_path,
            );
        }
        Cli::Update(cmd) => {
            fetcher::fetch_subject_messages(
                cmd.host,
                cmd.email,
                cmd.password,
                cmd.subject,
                &attachments_path,
            );
            let aggregation_data =
                data::aggregate_attachments(&attachments_path, &cmd.off_duty_keyword);
            io::write_parquet(aggregation_data, &aggregation_path);
        }
        Cli::Aggregate(cmd) => {
            let aggregation_data =
                data::aggregate_attachments(&attachments_path, &cmd.off_duty_keyword);
            io::write_parquet(aggregation_data, &aggregation_path);
        }

        Cli::OnDuty(cmd) => {
            let aggregation_data = io::read_parquet(&aggregation_path);
            let on_duty = data::calculate(aggregation_data);
            if cmd.print {
                println!("{on_duty}");
            }
            if cmd.export {
                let export = output_folder.join("on-duty.xlsx");
                io::write_excel(&export, &on_duty);
                println!(
                    "on-duty exported to {}",
                    export.to_string_lossy().to_string()
                )
            }
        }
    };
}
