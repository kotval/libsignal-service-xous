![cfg_attr(target_os = "none", no_main)]

mod api;
use api::*;
use crate::identy_key;
use num_traits::*;
use rand::::rngs::OsRng;
mod libsignal_protocol;

#[xous::xous_main]
fn xmain() -> ! {
    log_server::init_wait().unwrap();
    log::set_max_level(log::LevelFilter::Info);
    log::info!("my PID is {}", xous::process::id());
    let xns = xous_names::XousNames::new().unwrap();
    let sid = xns.register_name(api::SERVER_NAME, None).expect("can't register server");
    log::trace!("registered with NS -- {:?}", sid);
    loop {
        let msg = msg::xous::receive_message(sid).unwrap();
        match FromPrimitive::from_usize(msg.body.id()) {
            Some(Opcodes::GenerateIdentityKeyPair) =>{
                let mut buffer = unsafe {
                    Buffer::from_memory_message_mut(msg.body.memory_message_mut().unwrap)
                }
                let mut csprng = OsRng;
                let identity_keypair: IdentityKeyPair = libsignal_protocol::IdentityKeyPair::generate(&mut rng);
                buffer.replace(identity_keypair).expect("couldn't serialize return");
            }
        }
    }
}
