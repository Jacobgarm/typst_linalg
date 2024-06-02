use std::{fmt::Display, str::FromStr};

pub fn truncate_zeroes(num_str: String) -> String {
    let mut sep_found = false;
    let mut nonzero_found = false;
    let mut zeroes = 0;
    for (i, c) in num_str.chars().enumerate() {
        if c == '.' {
            sep_found = true;
        } else if "123456789".contains(c) {
            nonzero_found = true;
        } else if sep_found && nonzero_found && c == '0' {
            zeroes += 1;
            if zeroes == 10 {
                return (num_str[..=i]).to_owned();
            }
        } else {
            zeroes = 0;
        }
    }
    num_str
}

pub fn factorial(num: i64) -> i64 {
    (1..=num).product()
}

pub trait Scalar:
    num_traits::NumAssign + Display + Copy + std::ops::Neg<Output = Self> + FromStr + std::iter::Sum
{
}
impl<
        T: num_traits::NumAssign
            + Display
            + Copy
            + std::ops::Neg<Output = T>
            + FromStr
            + std::iter::Sum,
    > Scalar for T
{
}
