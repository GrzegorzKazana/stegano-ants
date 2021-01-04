pub struct UniquePair;

impl UniquePair {
    /*
     * generates deterministic an unique integer for pairs of integers
     * loosely based on Szudzik's pairing function http://szudzik.com/ElegantPairing.pdf
     * with the difference that input order does not matter
     */
    pub fn generate_key(from: u32, to: u32) -> u64 {
        let smaller = std::cmp::min(from, to) as u64;
        let bigger = std::cmp::max(from, to) as u64;

        bigger * bigger + smaller
    }

    /*
     * reverse function to generate_key
     * result order is not respected smaller id first
     * do not use for performance ciritical sections
     */
    pub fn decode_key(key: u64) -> (u32, u32) {
        let floor = (key as f64).sqrt().floor() as u64;

        ((key - floor * floor) as u32, floor as u32)
    }
}
