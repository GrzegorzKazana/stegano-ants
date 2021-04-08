use itertools::Itertools;
use std::iter::once;

use crate::ant_colony::graph::{AdjacencyListEntry, Graph, Node};
use crate::ant_colony::pheromone::Pheromone;
use crate::images::pixel_map::PixelMap;

use super::ImageGraphConverter;

pub struct ChunkToEdgeConverter {
    source_image: PixelMap,
    graph: Graph,
    n_chunks_x: usize,
    n_chunks_y: usize,
}

impl ChunkToEdgeConverter {
    pub fn new(pixel_map: &PixelMap, n_chunks_x: usize, n_chunks_y: usize, n_nodes: usize) -> Self {
        assert_eq!(n_nodes * (n_nodes - 1), n_chunks_x * n_chunks_y * 2);
        // let n_chunks_x = 75;
        // let n_chunks_y = 66; // multiplies to 4950, which is 100*99/2
        // let n_nodes = 100;

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
            .map(|chunk| 1.0 / (chunk.variance() + stability_factor!()));

        let indicies =
            (0..n_nodes_u32).flat_map(|from| (from + 1..n_nodes_u32).map(move |to| (from, to)));

        let edges = distances
            .zip_eq(indicies)
            .flat_map(|(distance, (from, to))| {
                let edge_a = AdjacencyListEntry::new(from, to, distance);
                let edge_b = AdjacencyListEntry::new(to, from, distance);

                once(edge_a).chain(once(edge_b))
            })
            .into_group_map_by(|edge| edge.from);

        let nodes = edges
            .into_iter()
            .map(|(id, adjacency_list)| Node { id, adjacency_list })
            .collect::<Vec<_>>();

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
