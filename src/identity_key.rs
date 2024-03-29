//To comply with GPL, I believe we'd need to put these definitions in a seperate crate so that they can be linked into the sigchat app. We have to
//make new types with the same layout so that we can deseialize the output data returned from the
//libsignal_protocol::IdentityKeyPair::generate call done in main.rs. If we could just link to the types, then we could also
//just link to the implementation. If we instead tried to do something like
// use libsignal_protocol::IdentityKeyPair
//#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
//pub struct XousIdentityKeyPair {
//    pub identity_key_pair: IdentityKeyPair
//}
// Then we could not share the definition of XousIdentityKeyPair without linking to this crate, which links to libsignal.
// This my argument against trying to write a service to do this. This is a lot of set up work to seperate libsignal, and it means ever
// upgrading libsignal will involve a lot of manual work that the rust type checker cannot help us with.

use rkyv;

//actually defined in libsignal/rust/protocol/src/identity_key.rs
#[derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize, Debug, PartialEq)]
pub struct IdentityKeyPair {
    pub identity_key: [u8; 32],
    pub private_key: [u8; 32],
}
impl IdentityKeyPair {
    pub fn new() -> IdentityKeyPair {
        IdentityKeyPair {
            identity_key: [0u8; 32],
            private_key: [0u8; 32],
        }
    }
}
#[derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize, Debug, PartialEq)]
pub struct IdentityKeyPairMessage {
    pub data: [u8; 64],
}
impl From<&ArchivedIdentityKeyPairMessage> for IdentityKeyPair {
    fn from(msg: &ArchivedIdentityKeyPairMessage) -> IdentityKeyPair {
        IdentityKeyPair {
            identity_key: msg.data[0..32].try_into().unwrap(),
            private_key: msg.data[32..64].try_into().unwrap(),
        }
    }
}
