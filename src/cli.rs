use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Divera Status Tracker (dst)", long_about = None)]
#[clap(propagate_version = true)]
pub enum Cli {
    /// Fetch all attachments
    Fetch(Fetch),
    /// Aggregate attachments
    Aggregate(Aggregate),
    /// Fetch all attachments and aggregate them
    Update(Update),
    /// Create on-duty table
    OnDuty(OnDuty),
    /// Create history plot
    History(History),
}

#[derive(Debug, Parser)]
pub struct Fetch {
    /// The email address used to fetch the attachments
    #[clap(long)]
    pub email: String,

    /// The host used to connect to
    #[clap(long)]
    pub host: String,

    /// The password that matches the email
    #[clap(long)]
    pub password: String,

    /// The email subject to filter the attachments
    #[clap(long)]
    pub subject: String,
}

#[derive(Debug, Parser)]
pub struct Aggregate {
    /// The keyword which is used in divera to indicate the off-duty status
    #[clap(long)]
    pub off_duty_keyword: String,
}

#[derive(Debug, Parser)]
pub struct Update {
    /// The email address used to fetch the attachments
    #[clap(long)]
    pub email: String,

    /// The host used to connect to
    #[clap(long)]
    pub host: String,

    /// The password that matches the email
    #[clap(long)]
    pub password: String,

    // The email subject to filter the attachments
    #[clap(long)]
    pub subject: String,

    /// The keyword which is used in divera to indicate the off-duty status
    #[clap(long)]
    pub off_duty_keyword: String,
}

#[derive(Debug, Parser)]
pub struct OnDuty {
    /// Print the overview
    #[clap(long)]
    pub print: bool,

    /// Export the overview as xlsx
    #[clap(long)]
    pub export: bool,
}

#[derive(Debug, Parser)]
pub struct History {
    /// Print the overview
    #[clap(long)]
    pub print: bool,

    /// Export the overview as html
    #[clap(long)]
    pub export: bool,

    /// The year
    #[clap(long, requires = "month")]
    pub year: Option<i32>,

    /// The month
    #[clap(long, requires = "year")]
    pub month: Option<u32>,
}
