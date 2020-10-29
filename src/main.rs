use std::env;
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;

use rosc::OscPacket;

fn main() {
    let args: Vec<String> = env::args().collect();

    let usage = format!("Usage: {} LISTEN_IP:PORT SEND_IP:PORT", &args[0]);
    if args.len() < 3 {
        println!("{}", usage);
        std::process::exit(1)
    }

    let listen_addr = SocketAddrV4::from_str(&args[1]).unwrap_or_else(|_| panic!("{}", &usage));
    let send_addr = SocketAddrV4::from_str(&args[2]).unwrap_or_else(|_| panic!("{}", &usage));

    let sock = UdpSocket::bind(listen_addr).unwrap();

    println!("Listening to {}", listen_addr);
    println!("Sending to {}", send_addr);

    let mut buf = [0u8; rosc::decoder::MTU];

    loop {
        let (size, addr) = sock.recv_from(&mut buf).unwrap();
        sock.send_to(&buf[..size], send_addr).unwrap();

        let mut packets = vec![rosc::decoder::decode(&buf[..size]).unwrap()];

        while !packets.is_empty() {
            let packet = packets.pop().unwrap();
            match packet {
                OscPacket::Message(msg) => {
                    println!("{}: {} ({:?})", addr, msg.addr, msg.args);
                }
                OscPacket::Bundle(bundle) => {
                    for p in bundle.content {
                        packets.push(p);
                    }
                }
            };
        }
    }
}
