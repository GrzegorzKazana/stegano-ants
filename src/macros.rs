#[macro_use]
macro_rules! iif {
    ($condition: expr, $_true: expr, $_false: expr) => {
        if $condition {
            $_true
        } else {
            $_false
        }
    };
}

#[cfg(test)]
#[macro_use]
macro_rules! assert_delta {
    ($x:expr, $y:expr, $d:expr) => {
        assert!($x - $y < $d || $y - $x < $d);
    };
}
