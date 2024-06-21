mod capture;
mod decode;
mod decrypt;
use capture::{capture_from_device, capture_from_file};
use decode::handle_ethernet_frame;
use pnet::packet::ethernet::EthernetPacket;

fn main() {
    // Capture from the default device
    let mut cap = capture_from_device();

    // Uncomment to read the capture from from a .cap file made with tcpdump
    // let mut cap = capture_from_file();

    cap.filter("port 443", true).unwrap();

    while let Ok(packet) = cap.next_packet() {
        let ethernet_packet = EthernetPacket::new(packet.data).unwrap();
        handle_ethernet_frame(&ethernet_packet);
    }
}
