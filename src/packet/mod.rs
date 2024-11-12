pub mod l2;
pub mod l3;

include!(concat!(env!("OUT_DIR"), "/packet.rs"));

impl Packet {

    pub fn process_l4(&mut self, _packet: &[u8]) -> Result<(), String> {
        Ok(())
    }
}