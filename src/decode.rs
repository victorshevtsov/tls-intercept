use std::net::IpAddr;

use crate::decrypt::handle_tls_packet;
use pnet::packet::{
    ethernet::{EtherTypes, EthernetPacket},
    ip::{IpNextHeaderProtocol, IpNextHeaderProtocols},
    ipv4::Ipv4Packet,
    ipv6::Ipv6Packet,
    tcp::TcpPacket,
    Packet,
};

pub fn handle_ethernet_frame(ethernet: &EthernetPacket) {
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => handle_ipv4_packet(ethernet),
        EtherTypes::Ipv6 => handle_ipv6_packet(ethernet),
        _ => {}
    }
}

fn handle_ipv4_packet(ethernet: &EthernetPacket) {
    let header = Ipv4Packet::new(ethernet.payload());
    if let Some(header) = header {
        handle_transport_protocol(
            IpAddr::V4(header.get_source()),
            IpAddr::V4(header.get_destination()),
            header.get_next_level_protocol(),
            header.payload(),
        );
    } else {
        println!("Malformed IPv4 Packet");
    }
}

fn handle_ipv6_packet(ethernet: &EthernetPacket) {
    let header = Ipv6Packet::new(ethernet.payload());
    if let Some(header) = header {
        handle_transport_protocol(
            IpAddr::V6(header.get_source()),
            IpAddr::V6(header.get_destination()),
            header.get_next_header(),
            header.payload(),
        );
    } else {
        println!("Malformed IPv6 Packet");
    }
}

fn handle_transport_protocol(
    source: IpAddr,
    destination: IpAddr,
    protocol: IpNextHeaderProtocol,
    packet: &[u8],
) {
    if protocol == IpNextHeaderProtocols::Tcp {
        handle_tcp_packet(source, destination, packet)
    }
}

fn handle_tcp_packet(source: IpAddr, destination: IpAddr, packet: &[u8]) {
    let tcp = TcpPacket::new(packet);
    if let Some(tcp) = tcp {
        if tcp.get_source() == 443 || tcp.get_destination() == 443 {
            println!(
                "TCP Packet: {}:{} > {}:{}; payload: {}",
                source,
                tcp.get_source(),
                destination,
                tcp.get_destination(),
                tcp.payload().len()
            );

            handle_tls_packet(tcp.payload());

            // println!("{:02x?}", tcp.payload());

            // let direction = match tcp.get_destination() {
            //     443 => Direction::ToServer,
            //     _ => Direction::ToClient,
            // };
            // let direction_u8 = match tcp.get_destination() {
            //     443 => 1,
            //     _ => 0,
            // };

            // let result = tls_parser.parse_tcp_level(tcp.payload(), direction);
            // println!("{:#?}", result);
            // // println!("{:#?}", tls_parser.sni);
            // println!("{:#?}", tls_parser.state);
            // println!("{:#?}", tls_parser.ssl_record_version);

            // let to_server = tcp.get_destination() == 443;
            // if tcp.payload().len() > 0 {
            //     handle_tls_packet(tcp.payload(), tls_state, to_server);
            // }

            // match std::str::from_utf8(tcp.payload()) {
            //     Ok(v) => println!("Payload: {:?}", v),
            //     Err(e) => println!("Invalid UTF-8 sequence: {}", e),
            // };
        }
    } else {
        println!("Malformed TCP Packet");
    }
}
