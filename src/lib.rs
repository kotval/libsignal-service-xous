mod api;
use api::{Opcodes};
mod identity_key;
use identity_key::*;
use xous_ipc::Buffer;

impl LibSignalService {
    pub fn generate_identy_key_pair(&self) -> Result<IdentityKeyPair, xous::Error> {
        let mut key_pair = IdentityKeyPair::new();

        let mut buf = Buffer::into_buf(key_pair).or(Err(xous::Error::InternalError))?;
        buf.lend_mut(self.conn, Opcodes::GenerateIdentityKeyPair.to_u32().unwrap()).map(|_| ())?;

        let response = buf.as_flat::<IdentityKeyPair, _>().unwrap();
        Ok(response)
    }
}
