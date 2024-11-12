use super::Packet as ProtoPacket;
use super::EthernetPacketType;
use log::error;
use pnet::packet::ethernet::{EtherType, EthernetPacket, EtherTypes};
use pnet::packet::Packet;

fn ethertype_to_proto(ether_type: EtherType) -> EthernetPacketType {
    match ether_type {
        // Internet Protocol version 4 (IPv4) \[RFC7042\].
        EtherTypes::Ipv4 => EthernetPacketType::EtherTypeIpv4,
        // Address Resolution Protocol (ARP) \[RFC7042\].
        EtherTypes::Arp => EthernetPacketType::EtherTypeArp,
        // Wake on Lan.
        EtherTypes::WakeOnLan => EthernetPacketType::EtherTypeWakeOnLan,
        // IETF TRILL Protocol \[IEEE\].
        EtherTypes::Trill => EthernetPacketType::EtherTypeTrill,
        // DECnet Phase IV.
        EtherTypes::DECnet => EthernetPacketType::EtherTypeDeCnet,
        // Reverse Address Resolution Protocol (RARP) \[RFC903\].
        EtherTypes::Rarp =>  EthernetPacketType::EtherTypeRarp,
        // AppleTalk - EtherTalk \[Apple\].
        EtherTypes::AppleTalk => EthernetPacketType::EtherTypeAppleTalk,
        // AppleTalk Address Resolution Protocol (AARP) \[Apple\].
        EtherTypes::Aarp => EthernetPacketType::EtherTypeAarp,
        // IPX \[Xerox\].
        EtherTypes::Ipx => EthernetPacketType::EtherTypeIpx,
        // QNX Qnet \[QNX Software Systems\].
        EtherTypes::Qnx => EthernetPacketType::EtherTypeQnx,
        // Internet Protocol version 6 (IPv6) \[RFC7042\].
        EtherTypes::Ipv6 => EthernetPacketType::EtherTypeIpv6,
        // Ethernet Flow Control \[IEEE 802.3x\].
        EtherTypes::FlowControl => EthernetPacketType::EtherTypeFlowControl,
        // CobraNet \[CobraNet\].
        EtherTypes::CobraNet => EthernetPacketType::EtherTypeCobraNet,
        // MPLS Unicast \[RFC 3032\].
        EtherTypes::Mpls => EthernetPacketType::EtherTypeMpls,
        // MPLS Multicast \[RFC 5332\].
        EtherTypes::MplsMcast => EthernetPacketType::EtherTypeMplsMcast,
        // PPPOE Discovery Stage \[RFC 2516\].
        EtherTypes::PppoeDiscovery => EthernetPacketType::EtherTypePppoeDiscovery,
        // PPPoE Session Stage \[RFC 2516\].
        EtherTypes::PppoeSession => EthernetPacketType::EtherTypePppoeSession,
        // VLAN-tagged frame (IEEE 802.1Q).
        EtherTypes::Vlan => EthernetPacketType::EtherTypeVlan,
        // Provider Bridging \[IEEE 802.1ad / IEEE 802.1aq\].
        EtherTypes::PBridge => EthernetPacketType::EtherTypePBridge,
        // Link Layer Discovery Protocol (LLDP) \[IEEE 802.1AB\].
        EtherTypes::Lldp => EthernetPacketType::EtherTypeLldp,
        // Precision Time Protocol (PTP) over Ethernet \[IEEE 1588\].
        EtherTypes::Ptp => EthernetPacketType::EtherTypePtp,
        // CFM / Y.1731 \[IEEE 802.1ag\].
        EtherTypes::Cfm => EthernetPacketType::EtherTypeCfm,
        // Q-in-Q Vlan Tagging \[IEEE 802.1Q\].
        EtherTypes::QinQ => EthernetPacketType::EtherTypeQinQ,
        _ => EthernetPacketType::EtherTypeUnknown
    }
}

impl ProtoPacket {
    pub fn process_l2(&mut self, packet: &[u8]) -> Result<Vec<u8>, String> {
        if let Some(ethernet_package) = EthernetPacket::new(packet) {
            self.l2.as_mut().unwrap().src_mac = ethernet_package.get_source().octets().to_vec();
            self.l2.as_mut().unwrap().dst_mac = ethernet_package.get_destination().octets().to_vec();
            self.l2.as_mut().unwrap().set_ether_type(ethertype_to_proto(ethernet_package.get_ethertype()));
            Ok(ethernet_package.payload().to_vec())
        } else {
            error!("Failed to parse L2 packet");
            Err(String::from("Failed to parse L2 packet"))
        }
    }

    
}