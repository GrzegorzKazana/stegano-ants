use clap::Clap;

use crate::common::utils::Capacity;

#[derive(Clap, Debug)]
#[clap(version = "1.0.0", author = "Grzegorz K. <kazana.grzegorz@gmail.com>")]
pub struct Opts {
    #[clap(long, default_value = "42", about = "rng seed")]
    pub seed: u64,

    #[clap(short, long, about = "amount of ants")]
    pub ants: usize,

    #[clap(short, long, about = "number of ant steps in single cycle")]
    pub steps: usize,

    #[clap(short, long, about = "dispatcher definition in format <type>:<args>")]
    pub dispatcher: String,

    #[clap(short, long, about = "updater type in format <type>:<args>")]
    pub updater: String,

    #[clap(short, long, about = "number of traning cycles")]
    pub cycles: Option<usize>,

    #[clap(
        long,
        about = "train until number of cycles does not provide improvement"
    )]
    pub stop_after: Option<usize>,

    #[clap(
        long,
        about = "dimension of the pheromone mask, directly affects graph size, height is calculated automatically"
    )]
    pub mask_width: Option<usize>,

    #[clap(long, about = "target capacity")]
    pub target_capacity: Option<Capacity>,

    #[clap(short, long)]
    pub quiet: bool,

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
