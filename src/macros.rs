/// Common constant for operations on floating numbers preventing
/// issues by division by extremely small numbers/zero
/// using macro instead of a constant, so it does not need to be imported in each file
/// also used when comparing float values using `assert_delta` and `assert_vec_delta` macros
#[macro_use]
macro_rules! stability_factor {
    () => {{
        1e-5f32
    }};
}

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
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

#[cfg(test)]
#[macro_use]
macro_rules! assert_delta {
    ($x:expr, $y:expr) => {
        assert!(
            ($y - $x).abs() <= 2.0 * stability_factor!(),
            format!("{} is not within delta to {}", $x, $y)
        );
    };
}

#[cfg(test)]
#[macro_use]
macro_rules! assert_vec_delta {
    ($xs:expr, $ys:expr) => {
        $xs.iter().zip($ys.iter()).for_each(|(x, y)| {
            assert_delta!(x, y);
        });
    };
}
