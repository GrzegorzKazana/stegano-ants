use itertools::Itertools;
use std::{collections::HashMap, iter::once};

use crate::ant_colony::graph::{AdjacencyListEntry, Graph, Node, NodeId};
use crate::ant_colony::pheromone::Pheromone;

use crate::images::image::Pixel;
use crate::images::pixel_map::{PixelMap, PixelMapWindows};

use super::ImageGraphConverter;

pub struct ChunkToEdgeConverter {
    pixel_map_windows: PixelMapWindows,
    // graph: Graph,
    n_nodes: usize,
    n_chunks_x: usize,
    n_chunks_y: usize,
    window_idx_to_node_pair: HashMap<usize, (NodeId, NodeId)>,
}

impl ChunkToEdgeConverter {
    pub fn new(pixel_map: &PixelMap, n_chunks_x: usize, n_chunks_y: usize, n_nodes: usize) -> Self {
        assert_eq!(n_nodes * (n_nodes - 1), n_chunks_x * n_chunks_y * 2);
        // let n_chunks_x = 75;
        // let n_chunks_y = 66; // multiplies to 4950, which is 100*99/2
        // let n_nodes = 100;

        ChunkToEdgeConverter {
            pixel_map_windows: pixel_map.windows(n_chunks_x, n_chunks_y),
            n_nodes,
            n_chunks_x,
            n_chunks_y,
            window_idx_to_node_pair: Self::build_window_idx_lookup(n_nodes),
        }
    }

    fn build_window_idx_lookup(n_nodes: usize) -> HashMap<usize, (NodeId, NodeId)> {
        let n_nodes_u32 = n_nodes as u32;

        (0..n_nodes_u32)
            .flat_map(|from| (from + 1..n_nodes_u32).map(move |to| (from, to)))
            .zip(0..)
            .map(|(nodes, idx)| (idx, nodes))
            .collect()
    }

    fn lookup_nodes_by_window_idx(&self, window_idx: usize) -> (NodeId, NodeId) {
        let nodes = self.window_idx_to_node_pair.get(&window_idx).cloned();

        debug_assert_ne!(nodes, None, "Failed to lookup nodes by window index");

        nodes.unwrap_or_default()
    }
}

impl ImageGraphConverter for ChunkToEdgeConverter {
    fn img_to_graph(&self) -> Graph {
        let distances = self
            .pixel_map_windows
            .iter()
            .map(|(idx, chunk)| (idx, 1.0 / (chunk.variance() + stability_factor!())));

        let edges = distances
            .flat_map(|(idx, distance)| {
                let (from, to) = self.lookup_nodes_by_window_idx(idx);
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

    fn visualize_pheromone(&self, pheromone: &Pheromone) -> PixelMap {
        let pheromone_norm = pheromone.normalize();

        self.pixel_map_windows.map_pixels(|px, window_idx| {
            let (from, to) = self.lookup_nodes_by_window_idx(window_idx);
            let edge_key = AdjacencyListEntry::get_key(from, to);
            let intensity = pheromone_norm.get_pheromone_for_edge(edge_key) * 255.0;

            Pixel::grey(px.x, px.y, intensity as u8)
        })
    }
}
