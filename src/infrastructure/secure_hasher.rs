use argon2::{Argon2, Params};
use argon2::Algorithm::Argon2id;
use lazy_static::lazy_static;

lazy_static! {
    static ref ARGON2_PARAMS: Params = Params::new(8192, 5, 1, Some(32)).unwrap();
    pub static ref ARGON2_HASHER: Argon2<'static> = Argon2::new(Argon2id, argon2::Version::V0x13, ARGON2_PARAMS.to_owned());
}