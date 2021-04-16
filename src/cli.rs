use clap::Clap;

use crate::common::utils::Capacity;

use crate::ant_colony::ant_dispatcher::DispatcherStringConfig;
use crate::ant_colony::pheromone_updater::UpdaterStringConfig;
use crate::images::image_graph_converter::ConverterStringConfig;

#[derive(Clap, Debug, Clone)]
#[clap(version = "1.0.0", author = "Grzegorz K. <kazana.grzegorz@gmail.com>")]
pub struct Opts {
    #[clap(long, default_value = "42", about = "rng seed")]
    pub seed: u64,

    #[clap(short, long, about = "amount of ants, by default number of nodes")]
    pub ants: Option<usize>,

    #[clap(
        short,
        long,
        about = "number of ant steps in single cycle, by default number of nodes"
    )]
    pub steps: Option<usize>,

    #[clap(short, long, about = "dispatcher definition in format <type>:<args>")]
    pub dispatcher: DispatcherStringConfig,

    #[clap(short, long, about = "updater type in format <type>:<args>")]
    pub updater: UpdaterStringConfig,

    #[clap(
        long,
        default_value = "i:spatial",
        about = "converter type in format <type>:<args>"
    )]
    pub converter: ConverterStringConfig,

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

    #[clap(long, about = "verbose filenames of output files")]
    pub verbose_files: bool,

    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap, Debug, Clone)]
pub enum SubCommand {
    #[clap()]
    Embed(EmbedCommand),
    #[clap()]
    Extract(ExtractCommand),
    #[clap()]
    Tsp(TspCommand),
}

#[derive(Clap, Debug, Clone)]
pub struct EmbedCommand {
    #[clap(short, long, about = "path to transport image")]
    pub image: String,

    #[clap(short, long, about = "path to .txt file with data")]
    pub data: String,
}

#[derive(Clap, Debug, Clone)]
pub struct ExtractCommand {
    #[clap(short, long, about = "path to transport image")]
    pub image: String,

    #[clap(short, long, about = "path to steganogram")]
    pub steg: String,
}

#[derive(Clap, Debug, Clone)]
pub struct TspCommand {
    #[clap(short, long, about = "number of graph nodes")]
    pub n_cities: Option<usize>,

    #[clap(short, long, about = "path to tsp graph csv")]
    pub graph: Option<String>,
}

impl ToString for Opts {
    fn to_string(&self) -> String {
        format!(
            "_a{}_s{}_D{}_U{}_C{}_c{}_m{}_t{}_",
            self.ants.unwrap_or_default(),
            self.steps.unwrap_or_default(),
            self.dispatcher.to_string(),
            self.updater.to_string(),
            self.converter.to_string(),
            self.cycles.unwrap_or_default(),
            self.mask_width.unwrap_or_default(),
            self.target_capacity
                .as_ref()
                .map(Capacity::to_string)
                .unwrap_or_default()
        )
    }
}
