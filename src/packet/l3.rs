use pnet::packet::{ip::IpNextHeaderProtocol, ipv4::Ipv4Packet, Packet};
use super::{EthernetPacketType, Packet as ProtoPacket, ProtocolType};
use log::{debug, info, warn};

fn protocol_to_proto(next_protocol: IpNextHeaderProtocol) -> Result<ProtocolType, String> {
    if let Ok(protocol_type) = ProtocolType::try_from(next_protocol.0 as i32) {
        Ok(protocol_type)
    } else {
        Err(String::from("Unknown network layer protocol"))
    }
}

impl ProtoPacket {
    fn process_ipv4(&mut self, packet: &[u8]) -> Result<Vec<u8>, String> {
        if let Some(ipv4_packet) = Ipv4Packet::new(packet) {
            let network_layer_protocol = protocol_to_proto(ipv4_packet.get_next_level_protocol())?;
            self.l3.as_mut().unwrap().set_protocol_type(network_layer_protocol);
            self.l3.as_mut().unwrap().dst_ip = ipv4_packet.get_destination().octets().to_vec();
            self.l3.as_mut().unwrap().src_ip = ipv4_packet.get_source().octets().to_vec();
            Ok(ipv4_packet.payload().to_vec())
        } else {
            Err(String::from("Error while parsing ipv4 packet"))
        }
        
    }

    pub fn process_l3(&mut self, packet: &[u8]) -> Result<Vec<u8>, String> {
        let ether_type: EthernetPacketType = self.l2.as_ref().unwrap().ether_type();
        println!("Furkan L3 {}", ether_type.as_str_name());
        debug!("EtherType: {}", ether_type.as_str_name());
        match ether_type {
            EthernetPacketType::EtherTypeIpv4 => self.process_ipv4(packet),
            EthernetPacketType::EtherTypeArp => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypeWakeOnLan => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypeTrill => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypeDeCnet => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypeRarp => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypeAppleTalk => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypeAarp => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypeIpx => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypeQnx => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypeIpv6 => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypeFlowControl => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypeCobraNet => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypeMpls => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypeMplsMcast => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypePppoeDiscovery => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypePppoeSession => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypeVlan => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypePBridge => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypeLldp => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypePtp => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypeCfm => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypeQinQ => Err(String::from("Not yet implemented")),
            EthernetPacketType::EtherTypeUnknown => Err(String::from("Not yet implemented")),
        }
    }

    
}