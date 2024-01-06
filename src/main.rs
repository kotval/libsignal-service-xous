#![cfg_attr(target_os = "none", no_main)]
mod api;
use api::*;
use lib;
use libsignal_protocol;
use num_traits::*;
use rand::Rng;
use xous_ipc::Buffer;
use xous_api_log_server as log_server;
use xous_api_names as xous_names;

fn main() -> ! {
    log_server::init_wait().unwrap();
    log::set_max_level(log::LevelFilter::Info);
    log::info!("my PID is {}", xous::process::id());
    let xns = xous_names::XousNames::new().unwrap();
    let sid = xns
        .register_name(api::SERVER_NAME, None)
        .expect("can't register server");
    log::trace!("registered with NS -- {:?}", sid);
    loop {
        let msg = xous::receive_message(sid).unwrap();
        match FromPrimitive::from_usize(msg.body.id()) {
            Some(Opcodes::GenerateIdentityKeyPair) => {
                let mut buffer = unsafe {
                    Buffer::from_memory_message_mut(msg.body.memory_message_mut().unwrap())
                };
                let mut rng = rand::thread_rng();
                let keypair: libsignal_protocol::KeyPair =
                    libsignal_protocol::IdentityKeyPair::generate(&mut rng).into();
                buffer
                    //TODO: can I implement rkyv::Serialize for libsignal_protocol::KeyPair?
                    .replace(keypair)
                    .expect("couldn't serialize return");
            }
        }
    }
}
