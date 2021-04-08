pub fn ceil_div(a: usize, b: usize) -> usize {
    a / b + iif!(a % b == 0, 0, 1)
}
