use rand::{prelude::StdRng, SeedableRng};

use crate::cli::{EmbedCommand, ExtractCommand, Opts, SubCommand};
use crate::common::cli_output::{CliOutput, CliOutputs};
use crate::common::errors::AppError;
use crate::common::execution_summary::{EmbeddingSummary, ExecutionSummary, ExtractionSummary};
use crate::common::utils::extend_basename;

use crate::ant_colony::ant_dispatcher::Dispatchers;
use crate::ant_colony::colony::{Colony, Config, ConfigurableColony, StepwiseParallelColony};
use crate::ant_colony::guided_configuration::GuidedConfiguration;
use crate::ant_colony::pheromone_updater::Updaters;
use crate::ant_colony::runner::ColonyRunner;

use crate::images::image::Image;
use crate::images::image_graph_converter::{EdgeChangeConverter, ImageGraphConverter};
use crate::images::pixel_map::PixelMap;

use crate::steganography::data::Data;
use crate::steganography::image_embedder::{EmbedInImage, MaskImageEmbedder};
use crate::steganography::quality_assessment::ImageMagick;

type AppResult<T> = Result<T, AppError>;

pub struct App<'a> {
    opts: Opts,
    cli: &'a CliOutputs,
}

impl<'a> App<'a> {
    pub fn new(opts: Opts, cli: &'a CliOutputs) -> Self {
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

        let (embedder, scaled_pheromone) =
            Self::prepare_embedder_and_mask(&self.opts, &pheromone_image);

        let mut bits_iter = data.iter_bits();
        let steganogram = embedder.embed(&mut bits_iter, &transport_image);

        let _ = self.save_pheromone_image(img_name, &pheromone_image)?;
        let _ = self.save_scaled_pheromone_image(img_name, &scaled_pheromone)?;
        let output_path = self.save_steg_image(img_name, &steganogram)?;

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
        let transport_image = self.load_image(&extract_opts.image)?;
        let steg_image = self.load_image(&extract_opts.steg)?;

        let pheromone_image = self.generate_pheromone_mask(&self.opts, &transport_image)?;

        let (embedder, _) = Self::prepare_embedder_and_mask(&self.opts, &pheromone_image);
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

        let downscaled_transport_image = Self::downscale_transport_image(opts, transport_image);

        let img_graph_converter = EdgeChangeConverter::new(&downscaled_transport_image);
        let graph = img_graph_converter.img_to_graph();

        let guide = GuidedConfiguration::from_graph(opts.ants, opts.steps, &graph);
        let ant_dispatcher = Self::parse_dispatcher(&opts, &guide)?;
        let pheromone_updater = Self::parse_pheromone_updater(&opts, &guide)?;

        let config = Config {
            ant_count: opts.ants,
            num_of_steps_per_cycle: opts.steps,
            pheromone_updater,
            ant_dispatcher,
            rng,
        };

        self.cli.print(&config);

        let colony = StepwiseParallelColony::new(config, &graph);
        let runner = ColonyRunner::new(colony, &graph, self.cli);
        let executed_runner = Self::execute_runner(runner, &opts)?;

        let pheromone = executed_runner.get_pheromone();
        let visualized_pheromone = img_graph_converter
            .visualize_pheromone(pheromone)
            .resize(transport_image.width, transport_image.height)
            .invert();

        Result::Ok(visualized_pheromone)
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

    fn parse_dispatcher(opts: &Opts, guide: &GuidedConfiguration) -> AppResult<Dispatchers> {
        Dispatchers::from_string(&opts.dispatcher, Option::Some(guide))
            .ok_or(format!("invalid dispatcher arg {}", opts.dispatcher))
            .map_err(AppError::IoError)
    }

    fn parse_pheromone_updater(opts: &Opts, guide: &GuidedConfiguration) -> AppResult<Updaters> {
        Updaters::from_string(&opts.updater, Option::Some(guide))
            .ok_or(format!("invalid updater arg {}", opts.updater))
            .map_err(AppError::IoError)
    }

    fn execute_runner<'b, C: Colony, IO: CliOutput>(
        runner: ColonyRunner<'b, C, IO>,
        opts: &Opts,
    ) -> AppResult<ColonyRunner<'b, C, IO>> {
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

    fn load_image(&self, path: &str) -> AppResult<PixelMap> {
        Image::load(path)
            .map_err(|_| format!("Failed to load image {}", path))
            .map(Image::into_pixel_map)
            .map_err(AppError::IoError)
    }

    fn save_image(&self, path: &str, pixel_map: &PixelMap) -> AppResult<String> {
        Image::from_pixel_map(&pixel_map)
            .save(path)
            .map(|_| path.to_owned())
            .map_err(|_| format!("Failed to save image: {}", path))
            .map_err(AppError::IoError)
    }

    fn save_steg_image(&self, name: &str, pixel_map: &PixelMap) -> AppResult<String> {
        extend_basename(name, "_steg")
            .ok_or(format!("Failed to generate file with extension."))
            .map_err(AppError::IoError)
            .and_then(|name_ext| self.save_image(&name_ext, pixel_map))
    }

    fn save_pheromone_image(&self, name: &str, pixel_map: &PixelMap) -> AppResult<String> {
        extend_basename(name, "_pher")
            .ok_or(format!("Failed to generate file with extension."))
            .map_err(AppError::IoError)
            .and_then(|name_ext| self.save_image(&name_ext, pixel_map))
    }

    fn save_scaled_pheromone_image(&self, name: &str, pixel_map: &PixelMap) -> AppResult<String> {
        extend_basename(name, "_pher_scaled")
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
