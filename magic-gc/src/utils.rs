use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn random_string(length: usize) -> String {
    let rng = thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
