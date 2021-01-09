use super::AdjacencyListEntry;

/// This is just an alias for any list of edges
/// which are unrelated in their essence (i.e. do not belong to same Route)
///
/// It is mostly used when edges from different routes are
/// processed at the same time
pub type RouteBatch = Vec<AdjacencyListEntry>;

/// Similar to above in function, but with the difference that some
/// routes may have finished before other ones.
///
/// For example, one among many ants could not find next edge to which it could transition to.
pub type RouteBatchWithHoles = Vec<Option<AdjacencyListEntry>>;
