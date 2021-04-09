use itertools::Itertools;
use std::collections::HashMap;

use crate::ant_colony::graph::NodeId;
use crate::ant_colony::pheromone::Pheromone;

use crate::images::image::Pixel;
use crate::images::pixel_map::{PixelMap, PixelMapWindows, WindowId};

use crate::common::utils::balanced_divisors;

use super::super::FromStrAndPixelMap;
use super::{SegmentDistances, SegmentId, SegmentToEdgeConverter};

/// Segments image into n_x_windows * n_y_windows non-overlapping windows.
///
/// Each window is mapped to an edge on the graph, the edge length
/// is 1/window.variance() which hopefully will make edges/windows
/// with complex structure more desirable.
pub struct WindowToEdgeConverter {
    pixel_map_windows: PixelMapWindows,
    n_nodes: usize,
    n_x_windows: usize,
    n_y_windows: usize,
    window_idx_to_node_pair: HashMap<WindowId, (NodeId, NodeId)>,
}

impl WindowToEdgeConverter {
    pub fn new(pixel_map: &PixelMap, n_nodes: usize) -> Self {
        let n_edges = n_nodes * (n_nodes - 1) / 2;
        let (n_y_windows, n_x_windows) = balanced_divisors(n_edges);
        println!("{}: {}x{}", n_edges, n_y_windows, n_x_windows);

        WindowToEdgeConverter {
            pixel_map_windows: pixel_map.windows(n_x_windows, n_y_windows),
            n_nodes,
            n_x_windows,
            n_y_windows,
            window_idx_to_node_pair: Self::build_segment_idx_node_lookup(n_nodes),
        }
    }

    pub fn new_without_resize(
        pixel_map: &PixelMap,
        n_x_windows: usize,
        n_y_windows: usize,
        n_nodes: usize,
    ) -> Self {
        assert_eq!(n_nodes * (n_nodes - 1), n_x_windows * n_y_windows * 2);

        WindowToEdgeConverter {
            pixel_map_windows: pixel_map.windows(n_x_windows, n_y_windows),
            n_nodes,
            n_x_windows,
            n_y_windows,
            window_idx_to_node_pair: Self::build_segment_idx_node_lookup(n_nodes),
        }
    }

    fn window_to_distance((idx, window): (WindowId, PixelMap)) -> (WindowId, f32) {
        (idx, 1.0 / (window.variance() + stability_factor!()))
    }
}

impl SegmentToEdgeConverter for WindowToEdgeConverter {
    fn distances(&self) -> Vec<SegmentDistances> {
        self.pixel_map_windows
            .iter()
            .map(Self::window_to_distance)
            .collect()
    }

    fn visualize_normalized_pheromone(&self, pheromone: &Pheromone) -> PixelMap {
        self.pixel_map_windows
            .map_pixels(|px, window_idx| self.map_pixel_with_segment_id(px, pheromone, window_idx))
    }

    fn lookup_nodes_by_segment_id(&self, segment_id: SegmentId) -> (NodeId, NodeId) {
        self.window_idx_to_node_pair
            .get(&segment_id)
            .cloned()
            .unwrap_or_default()
    }

    fn map_image_with_intensity_map(
        &self,
        intensity_by_segment_id: HashMap<SegmentId, u8>,
    ) -> PixelMap {
        self.pixel_map_windows.map_pixels(|px, window_idx| {
            Pixel::grey(
                px.x,
                px.y,
                intensity_by_segment_id
                    .get(&window_idx)
                    .cloned()
                    .unwrap_or_default(),
            )
        })
    }
}

impl FromStrAndPixelMap for WindowToEdgeConverter {
    fn from_str_and_pixel_map(pixel_map: &PixelMap, opts: &str) -> Option<Self> {
        let (n_nodes,): (usize,) = opts
            .splitn(1, ',')
            .map(str::parse)
            .filter_map(Result::ok)
            .collect_tuple()?;

        Some(WindowToEdgeConverter::new(pixel_map, n_nodes))
    }
}
