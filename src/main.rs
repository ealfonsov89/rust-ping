extern crate pnet;

use pnet::packet::icmp::MutableIcmpPacket;
use pnet::transport::{transport_channel, TransportChannelType, TransportSender};
use pnet::transport::TransportProtocol::Ipv4;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::net::{IpAddr, Ipv4Addr};
use serde::{Serialize, Deserialize};
use serde_json;

struct PingResult {
    time_lapse: i32,
    is_success: bool,
}

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
    icmp_packet.set_icmp_code(pnet::packet::icmp::IcmpCode(0));
    icmp_packet.set_checksum(pnet::packet::icmp::checksum(&icmp_packet.to_immutable()));
    return icmp_packet;
}

fn send_icmp_packet(mut icmp_sender: TransportSender, icmp_packet: MutableIcmpPacket<'_>, target_ip: IpAddr) -> bool {
    if let Err(error) = icmp_sender.send_to(icmp_packet, target_ip) {
        eprintln!("Error sending ICMP packet: {}", error);
        return false;
    } else {
        println!("Sent ICMP Echo Request to {}", target_ip);
        return true;
    }
}

fn configure_icmp_request() -> (IpAddr, TransportSender, MutableIcmpPacket<'_>) {
    let target_ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));
    let mut icmp_sender = get_icmp_sender();
    let mut packet = [0; 8];
    let icmp_packet = get_icmp_packet(packet);
    (target_ip, icmp_sender, icmp_packet)
}

fn icmp_statistics(target_ip: IpAddr, icmp_sender: TransportSender, icmp_packet: MutableIcmpPacket<'_>) -> PingResult {
    let start_time = std::time::Instant::now();
    let is_success = send_icmp_packet(icmp_sender, icmp_packet, target_ip);
    let end_time = std::time::Instant::now();
    let ping_result = PingResult {
        time_lapse: (end_time - start_time).as_millis() as i32,
        is_success: true,
    };
    return ping_result;
}


fn main() {
    let (target_ip, icmp_sender, icmp_packet) = configure_icmp_request();

    let mut ping_result_deque: VecDeque<PingResult> = VecDeque::new();

    let stdin = io::stdin();
    let mut input = String::new();
    
    loop {
        let ping_result = icmp_statistics(target_ip, icmp_sender, icmp_packet);
        println!("Ping Result: {:?} in {:?}s", ping_result.is_success, ping_result.time_lapse);
        ping_result_deque.push_back(ping_result);

        // Read input from the user
        stdin.lock().read_line(&mut input).unwrap();
        let input_value = input.trim();

        if input_value == "stop" {
            break;
        }

        input.clear(); 
    }

    let ping_result_deque_json = serde_json::to_value(&ping_result_deque).unwrap();
    let mut file = File::create("ping_results.json").unwrap();
    file.write_all(ping_result_deque_json.as_bytes()).unwrap();
}

