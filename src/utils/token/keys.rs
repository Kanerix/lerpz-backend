use std::{fs::File, io::Read, path::Path};

use jsonwebtoken::{DecodingKey, EncodingKey};
use lazy_static::lazy_static;

lazy_static! {
	pub static ref JWT_DECODE_KEY: DecodingKey = {
		let path = Path::new("./keys/ed25519_public.pem");
		let file = File::open(path).unwrap();

		let mut reader = std::io::BufReader::new(&file);
		let mut bytes = Vec::new();

		reader.read_to_end(&mut bytes).unwrap();
		DecodingKey::from_ed_pem(&bytes).unwrap()
	};
	pub static ref JWT_ENCODE_KEY: EncodingKey = {
		let path = Path::new("./keys/ed25519_private.pem");
		let file = File::open(path).unwrap();

		let mut reader = std::io::BufReader::new(&file);
		let mut bytes = Vec::new();

		reader.read_to_end(&mut bytes).unwrap();
		EncodingKey::from_ed_pem(&bytes).unwrap()
	};
}
