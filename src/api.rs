use num_traits::*;

pub const SERVER_NAME: &str = "libsignal_service";

#[derive(num_derive::FromPrimitive, num_derive::ToPrimitive, Debug)]
pub(crate) enum Opcodes {
    GenerateIdentityKeyPair,
}
