use clap::Clap;

#[derive(Clap, Debug)]
#[clap(version = "1.0.0", author = "Grzegorz K. <kazana.grzegorz@gmail.com>")]
pub struct Opts {
    #[clap(short, long, default_value = "42", about = "rng seed")]
    pub seed: u64,

    #[clap(short, long, about = "amount of ants")]
    pub ants: Option<usize>,

    #[clap(short, long, about = "ratio of ants to amount of nodes")]
    pub ratio: Option<f32>,

    #[clap(short, long, about = "dispatcher definition in format <type>:<args>")]
    pub dispatcher: String,

    #[clap(short, long, about = "updater type in format <type>:<args>")]
    pub updater: String,

    #[clap(short, long, about = "number of traning cycles")]
    pub cycles: Option<usize>,

    #[clap(
        short,
        long,
        about = "train until number of cycles does not provide improvement"
    )]
    pub stop_after: Option<usize>,

    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    #[clap()]
    Embed(EmbedCommand),
    #[clap()]
    Extract(ExtractCommand),
}

#[derive(Clap, Debug)]
pub struct EmbedCommand {
    #[clap(short, long, about = "path to transport image")]
    pub image: String,

    #[clap(short, long, about = "path to .txt file with data")]
    pub data: String,
}

#[derive(Clap, Debug)]
pub struct ExtractCommand {
    #[clap(short, long, about = "path to transport image")]
    pub image: String,

    #[clap(short, long, about = "path to steganogram")]
    pub steg: String,
}
