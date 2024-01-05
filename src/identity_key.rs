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

//actually defined in libsignal/rust/protocol/src/curve.rs
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
enum PublicKeyData {
    //DjbPublicKey([u8; curve25519::PUBLIC_KEY_LENGTH]) // defined in nonpublic module in libsignal/rust/protocol/curve/curve25519.rs
    DjbPublicKey([u8; 32])
}
//actually defined in libsignal/rust/protocol/src/curve.rs
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PublicKey {
    key: DjbPublicKey
}

//actually defined in libsignal/rust/protocol/src/curve.rs
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
enum PrivateKeyData{
    //DjbPrivateKey([u8; curve25519::PRIVATE_KEY_LENGTH])// defined in nonpublic module in libsignal/rust/protocol/curve/curve25519.rs
    DjbPrivateKey([u8; 32])
}

//actually defined in libsignal/rust/protocol/src/curve.rs
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PrivateKey {
    key: DjbPrivateKey
}

//actually defined in libsignal/rust/protocol/src/identity_key.rs
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct IdentityKey {
    key: PublicKey
}
//actually defined in libsignal/rust/protocol/src/identity_key.rs
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct IdentityKeyPair {
    pub identity_key: IdentityKey,
    pub private_key: PrivateKey,
}
