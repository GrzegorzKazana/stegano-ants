#![allow(dead_code)]
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "profiler")] {
        #[macro_use]
        // using extern instead of use, to import
        // flamer attribute once and do not need to conditionally
        // import it elsewhere
        extern crate flamer;

        use flame as f;
        use std::fs::File;
    }
}

use rand::{prelude::StdRng, SeedableRng};

#[macro_use]
mod macros;

mod ant_colony;
mod common;
mod images;
mod steganography;

use ant_colony::ant_dispatcher::BasicAntDispatcher;
use ant_colony::colony::{Colony, Config, ConfigurableColony, StepwiseParallelColony};
use ant_colony::graph::Graph;
use ant_colony::pheromone_updater::AveragePheromoneUpdater;
use ant_colony::runner::{ColonyRunner, CommandLine};

use images::image::Image;
use images::image_graph_converter::{EdgeChangeConverter, ImageGraphConverter};

use steganography::data::Data;
use steganography::image_embedder::{EmbedInImage, MaskImageEmbedder};

fn main() {
    let rng = StdRng::seed_from_u64(42);

    // let graph = Graph::from_neighbour_tuples(vec![
    //     (0, 1, 1.0),
    //     (0, 2, 2.0),
    //     (0, 3, 10.0),
    //     (1, 2, 2.0),
    //     (1, 3, 5.0),
    //     (2, 3, 6.0),
    // ]);
    // let graph = Graph::random_tsp_graph(&mut rng, 100);

    let transport_image = Image::load("./assets/images/sample1_xsmall.bmp")
        .unwrap()
        .into_pixel_map();

    let data = Data::from_file("./assets/data/lorem_ipsum.txt").unwrap();

    let img_graph_converter = EdgeChangeConverter::initialize(&transport_image);
    let graph = img_graph_converter.img_to_graph();

    let config = Config {
        ant_count: graph.get_amount_of_nodes(),
        num_of_steps_per_cycle: graph.get_amount_of_nodes() / 100,
        pheromone_updater: AveragePheromoneUpdater::new(1.0, 0.01, 0.1),
        ant_dispatcher: BasicAntDispatcher,
        rng,
    };

    let colony = StepwiseParallelColony::new(config, &graph);

    let runner = ColonyRunner::new(colony, &graph, CommandLine).train(1, 2);

    let pheromone = runner.get_pheromone();
    let pheromone_image = img_graph_converter.visualize_pheromone(pheromone);

    let embedder = MaskImageEmbedder::new(&pheromone_image);

    // .estimate_embeddable_bytes_and_transform(|capacity| {
    //     // > 1 data will not fit, we can upscale pheromone trails evenly
    //     // < 1 data will fit, but will be distributed unevenly
    //     // 1   data fits perfectly
    //     // ideal scenario is very close to 1 but below
    //     let fill_ratio = data.num_of_bytes() as f32 / capacity as f32;

    //     Option::Some(pheromone_image.scale(fill_ratio))
    // });

    let mut bits_iter = data.iter_bits();
    let steganogram = embedder.embed(&mut bits_iter, &transport_image);

    println!("{:?}", data.iter_bits().count());

    let remaining = bits_iter.count();
    println!(
        "Bit capacity: {:?}\nNum of data bits: {:?}\nRemaining bits: {:?}\nEmbedded bits: {:?}",
        embedder.estimate_embeddable_bytes() * 8,
        data.num_of_bits(),
        remaining,
        data.num_of_bits() - remaining
    );

    Image::from_pixel_map(&pheromone_image)
        .save("./assets/images/sample1_xsmall_pheromone.bmp")
        .unwrap();

    Image::from_pixel_map(&steganogram)
        .save("./assets/images/sample1_xsmall_steganogram.bmp")
        .unwrap();

    cfg_if! {
        if #[cfg(feature = "profiler")] {
            let latest_file_name_html = ".profiles/_latest.html";
            let latest_file_name_json = ".profiles/_latest.json";
            f::dump_html(File::create(latest_file_name_html).unwrap()).unwrap();
            f::dump_json(&mut File::create(latest_file_name_json).unwrap()).unwrap();
        }
    }
}

// TODO
// X. implement updaters: AveragePheromoneUpdater, CyclicalPheromoneUpdater (?), SystemPheromoneUpdater, MinMaxPheromoneUpdater (?)
// X. implement dispatchers: BiasedAntDispatcher, SystemAntDispatcher
// X. way to measure/track learning progress
// X. module for solving tsp
// X. research parallelization
//    5.1. Adapt colony to handle parallel cycle for each ant
// X. optimize weighted sampling in ant_dispatcher
// X. Get rid of few unwraps
