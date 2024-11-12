extern crate pnet;

use fdpi::packet::{Packet, L2, L3, L4};
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;

use std::env;

// Invoke as echo <interface name>
fn main() {
    let interface_name = env::args().nth(1).unwrap();
    let interface_names_match =
        |iface: &NetworkInterface| iface.name == interface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
                              .filter(interface_names_match)
                              .next()
                              .unwrap();

    // Create a new channel, dealing with layer 2 packets
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
    };

    log::set_max_level(log::LevelFilter::Debug);

    loop {
        match rx.next() {
            Ok(packet_raw) => {
                let mut packet = Packet{
                    l2: Some(L2::default()),
                    l3: Some(L3::default()),
                    l4: Some(L4::default())
                };
                let result = packet.process_l2(packet_raw)
                    .and_then(|l3_data| packet.process_l3(&l3_data))
                    .and_then(|l4_data| packet.process_l4(&l4_data));
                if let Ok(_) = result {
                    println!("{:?}", packet);
                } else {
                    continue;
                }
            },
            Err(e) => {
                // If an error occurs, we can handle it here
                panic!("An error occurred while reading: {}", e);
            }
        }

    }
}