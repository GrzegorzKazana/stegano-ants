/// Produces new value based on the previous one.
/// Stops when the value is accepted by a predicate
pub fn produce_until<T, F, P>(init: T, prod: F, accept: P) -> T
where
    F: Fn(T, usize) -> T,
    P: Fn(&T, usize) -> bool,
{
    let mut idx = 0;
    let mut val = init;

    loop {
        if accept(&val, idx) {
            return val;
        }

        val = prod(val, idx);
        idx += 1;
    }
}
