use itertools::Itertools;

use crate::ant_colony::graph::{AdjacencyListEntry, Graph, Node};
use crate::ant_colony::pheromone::Pheromone;
use crate::images::image::Pixel;
use crate::images::pixel_map::PixelMap;

use super::ImageGraphConverter;

pub struct ChunkToEdgeConverter {
    source_image: PixelMap,
    graph: Graph,
    n_chunks_x: usize,
    n_chunks_y: usize,
}

impl ChunkToEdgeConverter {
    pub fn new(pixel_map: &PixelMap) -> Self {
        let n_chunks_x = 75;
        let n_chunks_y = 66; // multiplies to 4950, which is 100*99/2
        let n_nodes = 100;

        ChunkToEdgeConverter {
            source_image: pixel_map.clone(),
            graph: Self::construct_graph(pixel_map, n_chunks_x, n_chunks_y, n_nodes),
            n_chunks_x,
            n_chunks_y,
        }
    }

    fn construct_graph(
        pixel_map: &PixelMap,
        n_chunks_x: usize,
        n_chunks_y: usize,
        n_nodes: usize,
    ) -> Graph {
        let n_nodes_u32 = n_nodes as u32;
        let distances = pixel_map
            .window_iter(n_chunks_x, n_chunks_y)
            .map(|chunk| 1.0 / chunk.variance())
            .chunks(n_nodes);

        let nodes = (0..n_nodes_u32)
            .zip_eq(distances.into_iter())
            .map(|(id, distances)| {
                let adjacency_list = (0..n_nodes_u32)
                    .zip_eq(distances)
                    .map(|(to, distance)| AdjacencyListEntry::new(id, to, distance))
                    .collect::<Vec<_>>();

                Node { id, adjacency_list }
            })
            .collect();

        Graph::from_node_vector(nodes)
    }
}

impl ImageGraphConverter for ChunkToEdgeConverter {
    fn img_to_graph(&self) -> Graph {
        self.graph.clone()
    }

    fn visualize_pheromone(&self, pheromone: &Pheromone) -> PixelMap {
        todo!()
    }
}
