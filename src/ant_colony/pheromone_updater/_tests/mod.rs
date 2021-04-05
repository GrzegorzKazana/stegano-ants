#[cfg(test)]
mod pheromone_updater_tests {
    use std::collections::HashMap;

    use crate::ant_colony::graph::{AdjacencyListEntry, EdgeKey, RouteCollection};
    use crate::ant_colony::pheromone::{Pheromone, PheromoneLevel};
    use crate::ant_colony::pheromone_updater::{
        AveragePheromoneUpdater, ConstantPheromoneUpdater, CyclicalPheromoneUpdater,
        PheromoneUpdater, SystemPheromoneUpdater,
    };

    fn get_edges() -> Vec<AdjacencyListEntry> {
        vec![
            AdjacencyListEntry {
                key: 0,
                from: 0,
                to: 0,
                distance: 1.0,
                visibility: 1.0,
            },
            AdjacencyListEntry {
                key: 1,
                from: 0,
                to: 0,
                distance: 2.0,
                visibility: 0.5,
            },
            AdjacencyListEntry {
                key: 2,
                from: 0,
                to: 0,
                distance: 3.0,
                visibility: 0.333,
            },
        ]
    }

    fn test_initialization<U: PheromoneUpdater>(updater: U) -> Pheromone {
        let edges = get_edges();
        let init_edges = vec![edges[0], edges[1], edges[2]];

        updater.initialize(Pheromone::new(), &init_edges)
    }

    fn test_step_update<U: PheromoneUpdater>(updater: U) -> Pheromone {
        let edges = get_edges();
        let init_edges = vec![edges[0], edges[1], edges[2]];
        let taken_edges = vec![edges[0], edges[0], edges[2]]
            .into_iter()
            .map(Option::Some)
            .collect::<Vec<_>>();

        let init_pheromone = updater.initialize(Pheromone::new(), &init_edges);

        updater.on_after_step(init_pheromone, &taken_edges)
    }

    fn test_cycle_update<U: PheromoneUpdater>(updater: U) -> Pheromone {
        let edges = get_edges();
        let init_edges = vec![edges[0], edges[1], edges[2]];

        // route lengths are (1 + 2; 2 + 3, 1 + 3)
        let taken_route = RouteCollection::new(3, 2)
            .add_steps(
                &vec![edges[0], edges[1], edges[0]]
                    .into_iter()
                    .map(Option::Some)
                    .collect::<Vec<_>>(),
            )
            .add_steps(
                &vec![edges[1], edges[2], edges[2]]
                    .into_iter()
                    .map(Option::Some)
                    .collect::<Vec<_>>(),
            );

        let init_pheromone = updater.initialize(Pheromone::new(), &init_edges);

        updater.on_after_cycle(init_pheromone, &taken_route)
    }

    #[test]
    fn constant_updater_correctly_inits_the_pheromone() {
        let updater = ConstantPheromoneUpdater::new(1.0, 0.2, 0.2);
        let pheromone = test_initialization(updater);

        let expected: HashMap<EdgeKey, PheromoneLevel> = map!(
            0 => 1.0,
            1 => 1.0,
            2 => 1.0
        );

        assert_eq!(*pheromone.get_values(), expected);
    }

    #[test]
    fn constant_updater_correctly_updates_the_pheromone_after_step() {
        let updater = ConstantPheromoneUpdater::new(1.0, 0.2, 0.2);
        let pheromone = test_step_update(updater);

        let expected: HashMap<EdgeKey, PheromoneLevel> = map!(
            // (1 * 0.8) + 0.2 + 0.2
            0 => 1.2,
            // (1 * 0.8)
            1 => 0.8,
            // (1 * 0.8) + 0.2
            2 => 1.0
        );

        assert_eq!(*pheromone.get_values(), expected);
    }

    #[test]
    fn constant_updater_correctly_updates_the_pheromone_after_cycle() {
        let updater = ConstantPheromoneUpdater::new(1.0, 0.2, 0.2);
        let pheromone = test_cycle_update(updater);

        let expected: HashMap<EdgeKey, PheromoneLevel> = map!(
            0 => 1.0,
            1 => 1.0,
            2 => 1.0
        );

        assert_eq!(*pheromone.get_values(), expected);
    }

    #[test]
    fn avg_updater_correctly_inits_the_pheromone() {
        let updater = AveragePheromoneUpdater::new(1.0, 0.2, 0.2);
        let pheromone = test_initialization(updater);

        let expected: HashMap<EdgeKey, PheromoneLevel> = map!(
            0 => 1.0,
            1 => 1.0,
            2 => 1.0
        );

        assert_eq!(*pheromone.get_values(), expected);
    }

    #[test]
    fn avg_updater_correctly_updates_the_pheromone_after_step() {
        let updater = AveragePheromoneUpdater::new(1.0, 0.2, 0.2);
        let pheromone = test_step_update(updater);

        let expected: HashMap<EdgeKey, PheromoneLevel> = map!(
            // (1 * 0.8) + 0.2 * (1 / 1) + 0.2 * (1 / 1)
            0 => 1.2,
            // (1 * 0.8)
            1 => 0.8,
            // (1 * 0.8) + 0.2 * (1 / 3)
            2 => 0.8 + 0.2 / 3.0
        );

        assert_eq!(*pheromone.get_values(), expected);
    }

    #[test]
    fn avg_updater_correctly_updates_the_pheromone_after_cycle() {
        let updater = AveragePheromoneUpdater::new(1.0, 0.2, 0.2);
        let pheromone = test_cycle_update(updater);

        let expected: HashMap<EdgeKey, PheromoneLevel> = map!(
            0 => 1.0,
            1 => 1.0,
            2 => 1.0
        );

        assert_eq!(*pheromone.get_values(), expected);
    }

    #[test]
    fn cyclical_updater_correctly_inits_the_pheromone() {
        let updater = CyclicalPheromoneUpdater::new(1.0, 0.2, 0.2, 2);
        let pheromone = test_initialization(updater);

        let expected: HashMap<EdgeKey, PheromoneLevel> = map!(
            0 => 1.0,
            1 => 1.0,
            2 => 1.0
        );

        assert_eq!(*pheromone.get_values(), expected);
    }

    #[test]
    fn cyclical_updater_correctly_updates_the_pheromone_after_step() {
        let updater = CyclicalPheromoneUpdater::new(1.0, 0.2, 0.2, 2);
        let pheromone = test_step_update(updater);

        let expected: HashMap<EdgeKey, PheromoneLevel> = map!(
            0 => 1.0,
            1 => 1.0,
            2 => 1.0
        );

        assert_eq!(*pheromone.get_values(), expected);
    }

    #[test]
    fn cyclical_updater_correctly_updates_the_pheromone_after_cycle() {
        let updater = CyclicalPheromoneUpdater::new(1.0, 0.2, 0.2, 2);
        let pheromone = test_cycle_update(updater);

        let expected: HashMap<EdgeKey, PheromoneLevel> = map!(
            // (1 * 0.8) + 0.2 * (1 / (1 + 2)) + 0.2 * (1 / (1 + 3))
            0 => 0.8 + 0.2 / 3.0 + 0.2 / 4.0,
            // (1 * 0.8) + 0.2 * (1 / (1 + 2)) + 0.2 * (1 / (2 + 3))
            1 => 0.8 + 0.2 / 3.0 + 0.2 / 5.0,
            // (1 * 0.8) + 0.2 * (1 / (2 + 3)) + 0.2 * (1 / (1 + 3))
            2 => 0.8 + 0.2 / 5.0 + 0.2 / 4.0
        );

        assert_eq!(*pheromone.get_values(), expected);
    }

    #[test]
    fn system_updater_correctly_inits_the_pheromone() {
        let updater = SystemPheromoneUpdater::new(1.0, 0.2);
        let pheromone = test_initialization(updater);

        let expected: HashMap<EdgeKey, PheromoneLevel> = map!(
            0 => 1.0,
            1 => 1.0,
            2 => 1.0
        );

        assert_eq!(*pheromone.get_values(), expected);
    }

    #[test]
    fn system_updater_correctly_updates_the_pheromone_after_step() {
        let updater = SystemPheromoneUpdater::new(1.0, 0.2);
        let pheromone = test_step_update(updater);

        let expected: HashMap<EdgeKey, PheromoneLevel> = map!(
            // (1 * 0.8) + 0.2 * 1 + 0.2 * 1
            0 => 1.2,
            // (1 * 0.8)
            1 => 0.8,
            // (1 * 0.8) + 0.2 * 1
            2 => 1.0
        );

        assert_eq!(*pheromone.get_values(), expected);
    }

    #[test]
    fn system_updater_correctly_updates_the_pheromone_after_cycle() {
        let updater = SystemPheromoneUpdater::new(1.0, 0.2);
        let pheromone = test_cycle_update(updater);

        // shortest route was (0, 1) and its length is (1 + 2)
        let expected: HashMap<EdgeKey, PheromoneLevel> = map!(
            // (1 * 0.8) + 0.2 * (1 / (1 + 2))
            0 => 0.8 + 0.2 / 3.0,
            // (1 * 0.8) + 0.2 * (1 / (1 + 2))
            1 => 0.8 + 0.2 / 3.0,
            // (1 * 0.8) + 0.2 * 0
            2 => 0.8
        );

        assert_eq!(*pheromone.get_values(), expected);
    }
}
