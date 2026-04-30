use rand::distr::Alphanumeric;
use rand::distr::uniform::SampleRange;
use rand::{RngExt, rng};

pub fn random_string<R: SampleRange<u8>>(length: R) -> String {
    let mut rng = rng();

    let length = rng.random_range(length);

    rng.sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect()
}
