use log::error;
use pnet::packet::{tcp::TcpPacket, Packet};

use super::{Packet as ProtoPacket, ProtocolType};

impl ProtoPacket {
    fn process_tcp(&mut self, packet: &[u8]) -> Result<Vec<u8>, String> {
        if let Some(tcp_packet) = TcpPacket::new(packet) {
            self.l4.as_mut().unwrap().dst_port = tcp_packet.get_destination() as u32;
            self.l4.as_mut().unwrap().src_port = tcp_packet.get_source() as u32;
            Ok(tcp_packet.payload().to_vec())
        } else {
            error!("Failed to parse L4:TCP packet");
            Err(String::from("Failed to parse L4:TCP packet"))
        }
    }

    pub fn process_l4(&mut self, packet: &[u8]) -> Result<Vec<u8>, String> {
        let protocol_type: ProtocolType = self.l3.as_ref().unwrap().protocol_type();
        match protocol_type {
            ProtocolType::IpProtocolHopopt => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIcmp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIgmp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolGgp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIpv4 => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolSt => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolTcp => self.process_tcp(packet),
            ProtocolType::IpProtocolCbt => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolEgp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIgp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolBbnRccMon => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolNvpIi => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolPup => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolArgus => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolEmcon => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolXnet => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolChaos => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolUdp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolMux => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolDcnMeas => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolHmp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolPrm => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolXnsIdp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolTrunk1 => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolTrunk2 => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolLeaf1 => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolLeaf2 => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolRdp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIrtp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIsoTp4 => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolNetblt => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolMfeNsp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolMeritInp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolDccp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolThreePc => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIdpr => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolXtp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolDdp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIdprCmtp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolTpPlusPlus => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIl => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIpv6 => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolSdrp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIpv6Route => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIpv6Frag => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIdrp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolRsvp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolGre => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolDsr => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolBna => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolEsp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolAh => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolINlsp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolSwipe => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolNarp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolMobile => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolTlsp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolSkip => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIcmpv6 => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIpv6NoNxt => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIpv6Opts => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolHostInternal => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolCftp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolLocalNetwork => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolSatExpak => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolKryptolan => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolRvd => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIppc => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolDistributedFs => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolSatMon => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolVisa => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIpcv => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolCpnx => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolCphb => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolWsn => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolPvp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolBrSatMon => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolSunNd => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolWbMon => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolWbExpak => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIsoIp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolVmtp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolSecureVmtp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolVines => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolTtpOrIptm => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolNsfnetIgp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolDgp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolTcf => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolEigrp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolOspfigP => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolSpriteRpc => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolLarp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolMtp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolAx25 => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIpIp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolMicp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolSccSp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolEtherip => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolEncap => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolPrivEncryption => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolGmtp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIfmp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolPnni => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolPim => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolAris => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolScps => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolQnx => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolAn => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIpComp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolSnp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolCompaqPeer => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIpxInIp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolVrrp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolPgm => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolZeroHop => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolL2tp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolDdx => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIatp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolStp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolSrp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolUti => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolSmp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolSm => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolPtp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIsisOverIpv4 => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolFire => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolCrtp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolCrudp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolSscopmce => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolIplt => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolSps => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolPipe => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolSctp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolFc => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolRsvpE2eIgnore => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolMobilityHeader => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolUdpLite => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolMplsInIp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolManet => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolHip => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolShim6 => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolWesp => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolRohc => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolTest1 => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolTest2 => Err(String::from("Not yet implemented")),
            ProtocolType::IpProtocolUnknown => Err(String::from("Not yet implemented")),
        }
    }

    
}