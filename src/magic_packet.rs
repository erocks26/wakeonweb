use std::net::IpAddr;
use rocket::tokio::net::UdpSocket;
use crate::mac_address::MacAddress;
use crate::WakeRequest;

fn magic_packet_constructor(mac: &MacAddress) -> Vec<u8> {
    let mut magic_packet: Vec<u8> = Vec::new();
    
    for _i in 0..6 {
        magic_packet.push(0xFF);
    }
    
    for _i in 0..16 {
        for i in 0..6 {
            magic_packet.push(mac.octets[i]);
        }
    }
    magic_packet
}

async fn send(frame: &[u8], address: &IpAddr, port: &usize) -> Result<String, String> {
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    
    socket.set_broadcast(true).unwrap();
    
    match socket.send_to(frame, address.to_string() + ":" + &port.to_string()).await {
        Ok(size) => Ok(size.to_string()),
        Err(e) => Err(format!("{}", e)),
    }
}

pub async fn magic(request: WakeRequest) -> Result<String, String> {
    send(
        &magic_packet_constructor(&request.mac), 
        &request.broadcast,
        &request.port
    ).await
}