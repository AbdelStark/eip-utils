use std::str::FromStr;

use num_bigint::BigUint;

pub fn option_to_big(value: Option<&str>) -> BigUint {
    str_to_big(value.unwrap())
}

pub fn str_to_big(value: &str) -> BigUint {
    BigUint::from_str(value).expect("invalid literal value specified, expected valid BigUint value")
}
