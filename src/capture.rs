use pcap::{Active, Capture, Device, Offline};

pub fn capture_from_device() -> Capture<Active> {
    let main_device = Device::lookup().unwrap();

    println!("cap device: {:?}", main_device);

    let cap = Capture::from_device(main_device.unwrap())
        .unwrap()
        .promisc(true)
        .snaplen(5000)
        .immediate_mode(true)
        .open()
        .unwrap();

    cap
}

pub fn capture_from_file() -> Capture<Offline> {
    let cap = Capture::from_file("/tmp/tcpdump.cap").unwrap();

    cap
}
