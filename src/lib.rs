mod api;
use api::Opcodes;
mod identity_key;
use identity_key::*;
use xous::CID;
use xous_ipc::Buffer;

pub struct LibSignalService {
    conn: CID,
}

impl LibSignalService {
    pub fn generate_identy_key_pair(&self) -> Result<IdentityKeyPair, xous::Error> {
        let mut key_pair = IdentityKeyPair::new();

        let mut buf = Buffer::into_buf(key_pair).or(Err(xous::Error::InternalError))?;
        buf.lend_mut(self.conn, Opcodes::GenerateIdentityKeyPair as u32)
            .map(|_| ())?;

        let response: &ArchivedIdentityKeyPairMessage =
            buf.as_flat::<IdentityKeyPairMessage, _>().unwrap();
        let key_pair_out = IdentityKeyPair::from(response);
        Ok(key_pair_out)
    }
}
