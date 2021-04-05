mod disk_io;
mod execution_summary;

use rand::{prelude::StdRng, SeedableRng};
use std::rc::Rc;

use crate::cli::{EmbedCommand, ExtractCommand, Opts, SubCommand, TspCommand};
use crate::common::cli_output::{CliOutput, CliOutputs};
use crate::common::errors::AppError;

use crate::ant_colony::ant_dispatcher::Dispatchers;
use crate::ant_colony::colony::{Colony, Config, ConfigurableColony, StepwiseParallelColony};
use crate::ant_colony::graph::Graph;
use crate::ant_colony::guiding_config::GuidingConfig;
use crate::ant_colony::pheromone_updater::Updaters;
use crate::ant_colony::runner::ColonyRunner;

use crate::images::image_graph_converter::{EdgeChangeConverter, ImageGraphConverter};
use crate::images::pixel_map::PixelMap;

use crate::steganography::image_embedder::{EmbedInImage, MaskImageEmbedder};
use crate::steganography::quality_assessment::ImageMagick;

use disk_io::DiskIo;
use execution_summary::{EmbeddingSummary, ExecutionSummary, ExtractionSummary, TspSummary};

pub type AppResult<T> = Result<T, AppError>;
type UnionizedColony = StepwiseParallelColony<Updaters, Dispatchers, StdRng>;
type UnionizedColonyRunner = ColonyRunner<UnionizedColony, CliOutputs>;

pub struct App {
    opts: Opts,
    cli: Rc<CliOutputs>,
}

impl App {
    pub fn new(opts: Opts, cli: Rc<CliOutputs>) -> Self {
        App { opts, cli }
    }

    pub fn run(&self) -> AppResult<ExecutionSummary> {
        match &self.opts.subcmd {
            SubCommand::Embed(embed_opts) => self.embed(embed_opts).map(ExecutionSummary::Embed),
            SubCommand::Extract(extract_opts) => {
                self.extract(extract_opts).map(ExecutionSummary::Extract)
            }
            SubCommand::Tsp(tsp_opts) => self.solve_tsp(&tsp_opts).map(ExecutionSummary::Tsp),
        }
    }

    fn embed(&self, embed_opts: &EmbedCommand) -> AppResult<EmbeddingSummary> {
        let img_name = &embed_opts.image;

        let transport_image = DiskIo::load_image(img_name)?;
        let data = DiskIo::load_data(&embed_opts.data)?;
        let pheromone_image = self.generate_pheromone_mask(&self.opts, &transport_image)?;

        let (embedder, scaled_pheromone) =
            Self::prepare_embedder_and_mask(&self.opts, &pheromone_image);

        let mut bits_iter = data.iter_bits();
        let steganogram = embedder.embed(&mut bits_iter, &transport_image);

        let _ = DiskIo::save_pheromone_image(img_name, &pheromone_image)?;
        let _ = DiskIo::save_scaled_pheromone_image(img_name, &scaled_pheromone)?;
        let output_path = DiskIo::save_steg_image(img_name, &steganogram)?;

        let summary = EmbeddingSummary::new(
            embedder.estimate_embeddable_bits(),
            data.num_of_bits(),
            bits_iter.count(),
            ImageMagick::mse(img_name, &output_path),
            ImageMagick::psnr(img_name, &output_path),
            ImageMagick::ssim(img_name, &output_path),
            ImageMagick::dssim(img_name, &output_path),
            ImageMagick::phash(img_name, &output_path),
        );

        Result::Ok(summary)
    }

    fn extract(&self, extract_opts: &ExtractCommand) -> AppResult<ExtractionSummary> {
        let transport_image = DiskIo::load_image(&extract_opts.image)?;
        let steg_image = DiskIo::load_image(&extract_opts.steg)?;

        let pheromone_image = self.generate_pheromone_mask(&self.opts, &transport_image)?;

        let (embedder, _) = Self::prepare_embedder_and_mask(&self.opts, &pheromone_image);
        let extracted = embedder.extract(&steg_image);

        let summary = ExtractionSummary::new(extracted);

        Result::Ok(summary)
    }

    fn solve_tsp(&self, tsp_opts: &TspCommand) -> AppResult<TspSummary> {
        let mut rng = StdRng::seed_from_u64(self.opts.seed);

        let graph = Self::read_tsp_graph(&mut rng, tsp_opts)?;
        let colony_runner = self.run_colony(&self.opts, rng, graph)?;
        let (last_cycle, last_epoch) = colony_runner
            .last_summaries()
            .ok_or(AppError::ColonyExecutionFailed)?;

        let summary = TspSummary::new(last_cycle, last_epoch);

        Result::Ok(summary)
    }

    fn generate_pheromone_mask(
        &self,
        opts: &Opts,
        transport_image: &PixelMap,
    ) -> AppResult<PixelMap> {
        let rng = StdRng::seed_from_u64(opts.seed);
        let downscaled_transport_image = Self::downscale_transport_image(opts, transport_image);

        let img_graph_converter = EdgeChangeConverter::new(&downscaled_transport_image);
        let graph = img_graph_converter.img_to_graph();

        let colony_runner = self.run_colony(opts, rng, graph)?;
        let colony = colony_runner.get_colony();
        let pheromone = colony.get_pheromone();
        let visualized_pheromone = img_graph_converter
            .visualize_pheromone(pheromone)
            .resize(transport_image.width, transport_image.height)
            .invert();

        Result::Ok(visualized_pheromone)
    }

    fn run_colony(
        &self,
        opts: &Opts,
        rng: StdRng,
        graph: Graph,
    ) -> AppResult<UnionizedColonyRunner> {
        let graph = Rc::new(graph);
        let ant_count = opts.ants.unwrap_or(graph.get_amount_of_nodes());
        let num_of_steps_per_cycle = opts.steps.unwrap_or(graph.get_amount_of_nodes());
        let guide = GuidingConfig::from_graph(
            ant_count,
            num_of_steps_per_cycle,
            opts.updater.clone(),
            &graph,
        );
        let ant_dispatcher = Self::parse_dispatcher(&opts, &guide)?;
        let pheromone_updater = Self::parse_pheromone_updater(&opts, &guide)?;

        let config = Config {
            ant_count,
            num_of_steps_per_cycle,
            pheromone_updater,
            ant_dispatcher,
            rng,
        };

        self.cli.print(&guide);
        self.cli.print(&config);

        let colony = StepwiseParallelColony::new(config, Rc::clone(&graph));
        let runner = ColonyRunner::new(colony, Rc::clone(&graph), Rc::clone(&self.cli));

        Self::execute_runner(runner, &opts)
    }

    fn downscale_transport_image(opts: &Opts, transport_image: &PixelMap) -> PixelMap {
        match opts.mask_width {
            Option::None => transport_image.clone(),
            Option::Some(target_width) => {
                if target_width >= transport_image.width {
                    return transport_image.clone();
                }

                let ratio = target_width as f32 / transport_image.width as f32;
                let target_height = (ratio * transport_image.height as f32) as usize;

                transport_image.resize(target_width, target_height)
            }
        }
    }

    fn parse_dispatcher(opts: &Opts, guide: &GuidingConfig) -> AppResult<Dispatchers> {
        Dispatchers::from_string_config(&opts.dispatcher, Option::Some(guide))
            .ok_or(format!("invalid dispatcher arg"))
            .map_err(AppError::IoError)
    }

    fn parse_pheromone_updater(opts: &Opts, guide: &GuidingConfig) -> AppResult<Updaters> {
        Updaters::from_string_config(&opts.updater, Option::Some(guide))
            .ok_or(format!("invalid updater arg"))
            .map_err(AppError::IoError)
    }

    fn execute_runner(
        runner: UnionizedColonyRunner,
        opts: &Opts,
    ) -> AppResult<UnionizedColonyRunner> {
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

    fn prepare_embedder_and_mask(
        opts: &Opts,
        pheromone_image: &PixelMap,
    ) -> (MaskImageEmbedder, PixelMap) {
        let embedder = MaskImageEmbedder::new(&pheromone_image);

        match opts.target_capacity {
            Option::Some(capacity) => {
                let scaled_image = embedder.scale_mask_to_fit(capacity.bits());

                (MaskImageEmbedder::new(&scaled_image), scaled_image)
            }
            Option::None => (embedder, pheromone_image.clone()),
        }
    }

    fn read_tsp_graph(rng: &mut StdRng, tsp_opts: &TspCommand) -> AppResult<Graph> {
        if let Option::Some(n_cities) = tsp_opts.n_cities {
            Some(Graph::random_tsp_graph(rng, n_cities))
        } else if let Option::Some(path) = tsp_opts.graph.as_ref() {
            let csv = DiskIo::load_csv(path)?;
            Some(Graph::from_coordinate_csv(&csv))
        } else {
            None
        }
        .ok_or(format!("you must specify n-cities or graph"))
        .map_err(AppError::IoError)
    }
}
