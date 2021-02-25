use rand::{prelude::StdRng, SeedableRng};

use crate::cli::{EmbedCommand, ExtractCommand, Opts, SubCommand};
use crate::common::cli_output::{CliOutput, CliOutputs};
use crate::common::errors::AppError;
use crate::common::execution_summary::{EmbeddingSummary, ExecutionSummary, ExtractionSummary};
use crate::common::utils::extend_basename;

use crate::ant_colony::ant_dispatcher::Dispatchers;
use crate::ant_colony::colony::{Colony, Config, ConfigurableColony, StepwiseParallelColony};
use crate::ant_colony::pheromone_updater::Updaters;
use crate::ant_colony::runner::ColonyRunner;

use crate::images::image::Image;
use crate::images::image_graph_converter::{EdgeChangeConverter, ImageGraphConverter};
use crate::images::pixel_map::PixelMap;

use crate::steganography::data::Data;
use crate::steganography::image_embedder::{EmbedInImage, MaskImageEmbedder};

type AppResult<T> = Result<T, AppError>;

pub struct App {
    opts: Opts,
    cli: CliOutputs,
}

impl App {
    pub fn new(opts: Opts) -> Self {
        let cli = CliOutputs::from_bool(opts.quiet);

        App { opts, cli }
    }

    pub fn run(&self) -> AppResult<ExecutionSummary> {
        match &self.opts.subcmd {
            SubCommand::Embed(embed_opts) => self.embed(embed_opts).map(ExecutionSummary::Embed),
            SubCommand::Extract(extract_opts) => {
                self.extract(extract_opts).map(ExecutionSummary::Extract)
            }
        }
    }

    fn embed(&self, embed_opts: &EmbedCommand) -> AppResult<EmbeddingSummary> {
        let img_name = &embed_opts.image;

        let transport_image = self.load_image(img_name)?;
        let data = self.load_data(&embed_opts.data)?;
        let pheromone_image = self.generate_pheromone_mask(&self.opts, &transport_image)?;

        let embedder = MaskImageEmbedder::new(&pheromone_image);

        let mut bits_iter = data.iter_bits();
        let steganogram = embedder.embed(&mut bits_iter, &transport_image);

        self.save_pheromone_image(img_name, &pheromone_image)?;
        self.save_steg_image(img_name, &steganogram)?;

        let summary = EmbeddingSummary::new(
            embedder.estimate_embeddable_bits(),
            data.num_of_bits(),
            bits_iter.count(),
        );

        Result::Ok(summary)
    }

    fn extract(&self, extract_opts: &ExtractCommand) -> AppResult<ExtractionSummary> {
        let transport_image = self.load_image(&extract_opts.image)?;
        let steg_image = self.load_image(&extract_opts.steg)?;

        let pheromone_image = self.generate_pheromone_mask(&self.opts, &transport_image)?;

        let embedder = MaskImageEmbedder::new(&pheromone_image);
        let extracted = embedder.extract(&steg_image);

        let summary = ExtractionSummary::new(extracted);

        Result::Ok(summary)
    }

    fn generate_pheromone_mask(
        &self,
        opts: &Opts,
        transport_image: &PixelMap,
    ) -> AppResult<PixelMap> {
        let rng = StdRng::seed_from_u64(opts.seed);

        let img_graph_converter = EdgeChangeConverter::initialize(&transport_image);
        let graph = img_graph_converter.img_to_graph();

        let ant_dispatcher = Self::parse_dispatcher(&opts)?;
        let pheromone_updater = Self::parse_pheromone_updater(&opts)?;

        let config = Config {
            ant_count: opts.ants,
            num_of_steps_per_cycle: opts.steps,
            pheromone_updater,
            ant_dispatcher,
            rng,
        };

        self.cli.print(&config);

        let colony = StepwiseParallelColony::new(config, &graph);
        let runner = ColonyRunner::new(colony, &graph, &self.cli);
        let executed_runner = Self::execute_runner(runner, &opts)?;

        let pheromone = executed_runner.get_pheromone();

        Result::Ok(img_graph_converter.visualize_pheromone(pheromone))
    }

    fn parse_dispatcher(opts: &Opts) -> AppResult<Dispatchers> {
        Dispatchers::from_string(&opts.dispatcher)
            .ok_or(format!("invalid dispatcher arg {}", opts.dispatcher))
            .map_err(AppError::IoError)
    }

    fn parse_pheromone_updater(opts: &Opts) -> AppResult<Updaters> {
        Updaters::from_string(&opts.updater)
            .ok_or(format!("invalid updater arg {}", opts.updater))
            .map_err(AppError::IoError)
    }

    fn execute_runner<'a, C: Colony, IO: CliOutput>(
        runner: ColonyRunner<'a, C, IO>,
        opts: &Opts,
    ) -> AppResult<ColonyRunner<'a, C, IO>> {
        if let Option::Some(n_cycles) = opts.cycles {
            Option::Some(runner.train(1, n_cycles))
        } else if let Option::Some(n_until) = opts.stop_after {
            Option::Some(runner.train_n_until_no_improvement(n_until))
        } else {
            Option::None
        }
        .ok_or(format!("you must specify cycles or stop_after"))
        .map_err(AppError::IoError)
    }

    fn load_image(&self, path: &str) -> AppResult<PixelMap> {
        Image::load(path)
            .map_err(|_| format!("Failed to load image {}", path))
            .map(Image::into_pixel_map)
            .map_err(AppError::IoError)
    }

    fn save_image(&self, path: &str, pixel_map: &PixelMap) -> AppResult<()> {
        Image::from_pixel_map(&pixel_map)
            .save(path)
            .map_err(|_| format!("Failed to save image: {}", path))
            .map_err(AppError::IoError)
    }

    fn save_steg_image(&self, name: &str, pixel_map: &PixelMap) -> AppResult<()> {
        extend_basename(name, "_steg")
            .ok_or(format!("Failed to generate file with extension."))
            .map_err(AppError::IoError)
            .and_then(|name_ext| self.save_image(&name_ext, pixel_map))
    }

    fn save_pheromone_image(&self, name: &str, pixel_map: &PixelMap) -> AppResult<()> {
        extend_basename(name, "_pher")
            .ok_or(format!("Failed to generate file with extension."))
            .map_err(AppError::IoError)
            .and_then(|name_ext| self.save_image(&name_ext, pixel_map))
    }

    fn load_data(&self, path: &str) -> AppResult<Data> {
        Data::from_file(path)
            .map_err(|_| format!("Failed to load data {}", path))
            .map_err(AppError::IoError)
    }
}
