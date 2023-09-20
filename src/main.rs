extern crate pnet;

use pnet::packet::icmp::MutableIcmpPacket;
use pnet::transport::{transport_channel, TransportChannelType, TransportSender};
use pnet::transport::TransportProtocol::Ipv4;
use std::net::{IpAddr, Ipv4Addr};

fn get_icmp_sender() -> pnet::transport::TransportSender {
    let mut icmp_sender = match transport_channel(2048, TransportChannelType::Layer4(Ipv4(pnet::packet::ip::IpNextHeaderProtocols::Icmp))) {
        Ok((sender, _)) => sender,
        Err(err) => panic!("{:?}", err),
    };
    return icmp_sender;
}

fn get_icmp_packet(mut packet: [u8; 8]) -> MutableIcmpPacket<'_> {
    let mut icmp_packet = MutableIcmpPacket::new(&mut packet).expect("Failed to create ICMP packet");
    icmp_packet.set_icmp_type(pnet::packet::icmp::IcmpType(8));
    // ICMP Echo Request
    icmp_packet.set_icmp_code(pnet::packet::icmp::IcmpCode(0));
    icmp_packet.set_checksum(pnet::packet::icmp::checksum(&icmp_packet.to_immutable()));
    return icmp_packet;
}

fn send_icmp_packet(mut icmp_sender: TransportSender, icmp_packet: MutableIcmpPacket<'_>, target_ip: IpAddr) -> u8 {
    if let Err(error) = icmp_sender.send_to(icmp_packet, target_ip) {
        eprintln!("Error sending ICMP packet: {}", error);
        return 1;
    } else {
        println!("Sent ICMP Echo Request to {}", target_ip);
        return 0;
    }
}

fn send_ping_request(target_ip: IpAddr) -> u8 {
    let mut icmp_sender = get_icmp_sender();
    let mut packet = [0; 8];
    let icmp_packet = get_icmp_packet(packet);
    return send_icmp_packet(icmp_sender, icmp_packet, target_ip);
}



fn main() {
    let target_ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)); // Replace with your target IP
    send_ping_request(target_ip);
}
