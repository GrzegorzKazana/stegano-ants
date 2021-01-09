use std::cmp::Ordering;

/// Encapsulates potentially unsafe scenario of unwrapping result of comparing floats,
/// which may fail for NaNs. Will panic in debug, return Equal in release.
///
/// f32 and f64 implement only PartialOrd, but most sorting/min/max functions
/// expect items implmementing the Ord trait or a appropiate function
pub fn compare_float<T: PartialOrd>(a: &T, b: &T) -> Ordering {
    let maybe_ord = a.partial_cmp(b);

    debug_assert_ne!(
        maybe_ord,
        Option::None,
        "float_comparison failed - check for NaNs"
    );

    maybe_ord.unwrap_or(Ordering::Equal)
}
