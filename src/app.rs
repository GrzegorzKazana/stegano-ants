use rand::{prelude::StdRng, SeedableRng};
use std::path::Path;

use crate::cli::{EmbedCommand, ExtractCommand, Opts, SubCommand};
use crate::common::errors::AppError;

use crate::ant_colony::ant_dispatcher::Dispatchers;
use crate::ant_colony::colony::{Config, ConfigurableColony, StepwiseParallelColony};
use crate::ant_colony::pheromone_updater::Updaters;
use crate::ant_colony::runner::{ColonyRunner, CommandLine};

use crate::images::image::Image;
use crate::images::image_graph_converter::{EdgeChangeConverter, ImageGraphConverter};
use crate::images::pixel_map::PixelMap;

use crate::steganography::data::Data;
use crate::steganography::image_embedder::{EmbedInImage, MaskImageEmbedder};

type AppResult<T> = Result<T, AppError>;

pub struct App {
    opts: Opts,
    data_base_path: String,
    image_base_path: String,
}

impl App {
    pub fn new(opts: Opts) -> Self {
        App {
            opts,
            data_base_path: String::from("./assets/data"),
            image_base_path: String::from("./assets/images"),
        }
    }

    pub fn run(&self) -> AppResult<String> {
        match &self.opts.subcmd {
            SubCommand::Embed(embed_opts) => self.embed(embed_opts),
            SubCommand::Extract(extract_opts) => self.extract(extract_opts),
        }
    }

    fn embed(&self, embed_opts: &EmbedCommand) -> AppResult<String> {
        let img_name = &embed_opts.image;

        let transport_image = self.load_image(img_name)?;
        let data = self.load_data(&embed_opts.data)?;
        let pheromone_image = Self::generate_pheromone_mask(&self.opts, &transport_image)?;

        let embedder = MaskImageEmbedder::new(&pheromone_image);

        let mut bits_iter = data.iter_bits();
        let steganogram = embedder.embed(&mut bits_iter, &transport_image);

        let remaining = bits_iter.count();

        self.save_pheromone_image(img_name, &pheromone_image)?;
        self.save_steg_image(img_name, &steganogram)?;

        Result::Ok(format!(
            "Bit capacity: {:?}\nNum of data bits: {:?}\nRemaining bits: {:?}\nEmbedded bits: {:?}",
            embedder.estimate_embeddable_bits(),
            data.num_of_bits(),
            remaining,
            data.num_of_bits() - remaining
        ))
    }

    fn extract(&self, extract_opts: &ExtractCommand) -> AppResult<String> {
        let transport_image = self.load_image(&extract_opts.image)?;
        let steg_image = self.load_image(&extract_opts.steg)?;

        let pheromone_image = Self::generate_pheromone_mask(&self.opts, &transport_image)?;

        let embedder = MaskImageEmbedder::new(&pheromone_image);
        let extracted = embedder.extract(&steg_image);

        Result::Ok(format!("Extracted:\n{}", extracted.to_string()))
    }

    fn generate_pheromone_mask(opts: &Opts, transport_image: &PixelMap) -> AppResult<PixelMap> {
        let rng = StdRng::seed_from_u64(opts.seed);

        let img_graph_converter = EdgeChangeConverter::initialize(&transport_image);
        let graph = img_graph_converter.img_to_graph();

        let ant_dispatcher = Dispatchers::from_string(&opts.dispatcher)
            .ok_or(format!("invalid dispatcher arg {}", opts.dispatcher))
            .map_err(AppError::IoError)?;

        let pheromone_updater = Updaters::from_string(&opts.updater)
            .ok_or(format!("invalid updater arg {}", opts.updater))
            .map_err(AppError::IoError)?;

        let ant_count = opts
            .ants
            .or_else(|| {
                opts.ratio
                    .map(|ratio| (ratio * graph.get_amount_of_nodes() as f32) as usize)
            })
            .ok_or(format!(
                "you must specify amount of ants or ratio to graph nodes"
            ))
            .map_err(AppError::IoError)?;

        let num_of_steps_per_cycle = graph.get_amount_of_nodes() / ant_count;

        let config = Config {
            ant_count,
            num_of_steps_per_cycle,
            pheromone_updater,
            ant_dispatcher,
            rng,
        };

        let colony = StepwiseParallelColony::new(config, &graph);

        let runner = ColonyRunner::new(colony, &graph, CommandLine);

        let executed_runner = match (opts.cycles, opts.stop_after) {
            (Option::Some(n_cycles), _) => Option::Some(runner.train(1, n_cycles)),
            (_, Option::Some(n_until)) => {
                Option::Some(runner.train_n_until_no_improvement(n_until))
            }
            _ => Option::None,
        }
        .ok_or(format!("you must specify cycles or stop_after"))
        .map_err(AppError::IoError)?;

        let pheromone = executed_runner.get_pheromone();

        Result::Ok(img_graph_converter.visualize_pheromone(pheromone))
    }

    fn load_image(&self, name: &str) -> AppResult<PixelMap> {
        Path::new(&self.image_base_path)
            .join(name)
            .to_str()
            .ok_or(format!("invalid image path {}", name))
            .and_then(|path| {
                Image::load(path).map_err(|_| format!("Failed to load image {}", path))
            })
            .map(Image::into_pixel_map)
            .map_err(AppError::IoError)
    }

    fn save_image(&self, name: &str, pixel_map: &PixelMap) -> AppResult<()> {
        Path::new(&self.image_base_path)
            .join(name)
            .to_str()
            .ok_or(format!("invalid image path {}", name))
            .and_then(|path| {
                Image::from_pixel_map(&pixel_map)
                    .save(path)
                    .map_err(|_| format!("Failed to save image: {}", path))
            })
            .map_err(AppError::IoError)
    }

    fn save_steg_image(&self, name: &str, pixel_map: &PixelMap) -> AppResult<()> {
        Self::extend_basename(name, "_steg")
            .and_then(|name_ext| self.save_image(&name_ext, pixel_map))
    }

    fn save_pheromone_image(&self, name: &str, pixel_map: &PixelMap) -> AppResult<()> {
        Self::extend_basename(name, "_pher")
            .and_then(|name_ext| self.save_image(&name_ext, pixel_map))
    }

    fn extend_basename(name: &str, infix: &str) -> AppResult<String> {
        let basename = Path::new(name).file_stem().and_then(|a| a.to_str());
        let extension = Path::new(name).extension().and_then(|a| a.to_str());

        basename
            .zip(extension)
            .map(|(base, ext)| format!("{}{}.{}", base, infix, ext))
            .ok_or(format!("Failed to generate file with extension."))
            .map_err(AppError::IoError)
    }

    fn load_data(&self, name: &str) -> AppResult<Data> {
        Path::new(&self.data_base_path)
            .join(name)
            .to_str()
            .ok_or(format!("invalid data path {}", name))
            .and_then(|path| {
                Data::from_file(path).map_err(|_| format!("Failed to load data {}", path))
            })
            .map_err(AppError::IoError)
    }
}
