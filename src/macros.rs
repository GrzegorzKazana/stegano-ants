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
    ($x:expr, $y:expr, $d:expr) => {
        assert!(
            ($y - $x).abs() < $d,
            format!("{} is not within delta to {}", $x, $y)
        );
    };
}

#[cfg(test)]
#[macro_use]
macro_rules! assert_vec_delta {
    ($xs:expr, $ys:expr, $d:expr) => {
        $xs.iter().zip($ys.iter()).for_each(|(x, y)| {
            assert_delta!(x, y, $d);
        });
    };
}
