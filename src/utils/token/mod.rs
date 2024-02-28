pub mod claims;
pub mod decode;
pub mod encode;
pub mod keys;

use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn generate_refresh_token() -> String {
	let rng = thread_rng();
	rng.sample_iter(&Alphanumeric)
		.take(32)
		.map(char::from)
		.collect()
}
