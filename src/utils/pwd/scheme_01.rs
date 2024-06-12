use super::Scheme;

pub struct Scheme01;

impl Scheme for Scheme01 {
	fn hash(pwd: &[u8]) -> super::error::Result<&str> {
		todo!()
	}

	fn verify(pwd: &[u8]) -> bool {
		todo!()
	}
}
