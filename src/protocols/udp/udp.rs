/*
 * @Author: uyplayer
 * @Date: 2023/9/14 11:47
 * @Email: uyplayer@qq.com
 * @File: udp
 * @Software: CLion
 * @Dir: proxy_simulator / src/protocols/udp
 * @Project_Name: proxy_simulator
 * @Description:
 */


use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UdpSocket};
use crate::helper::protocol_info::PackageProtocol;

pub async fn udp_handler(mut stream: TcpStream, protocol: PackageProtocol, config: HashMap<&str, String>){
    // stream must be udp type
    let (mut source_read, mut source_write) = stream.split();
    let mut buffer = [0; 1024];
    let len = &source_read.read(&mut buffer).await.expect("peek failed");
    let request_str = String::from_utf8_lossy(&buffer[..*len]);
    println!("{}",request_str);
    let remote_addr = protocol.destination_host.unwrap();
    let remote_port = protocol.destination_port.unwrap();
    let sock = UdpSocket::bind("0.0.0.0:8090").await.unwrap();
    let remote_addr = format!("{}:{}",remote_addr,remote_port);
    sock.connect(&remote_addr).await.expect("TODO: panic message");
    let response = "HTTP/1.1 200 Connection Established\r\n\r\n";
    if let Err(e) = source_write.write_all(response.as_ref()).await {
        log::error!("Error sending 200 response to client: {:?}", e);
        return;
    }
    // let mut buf = [0; 100000];
    // loop {
    //     let len = source_read.read(&mut buf).await.expect("TODO: panic message");
    //     println!("receiver from client: {:?}", &buf[0..len]);
    //     let send_len = sock.send(&buf[0..len]).await.expect("TODO: panic message");
    //     println!("send to remote: {:?}", &buf[0..send_len]);
    //     match sock.recv_from(&mut buf).await {
    //         Ok((recv_len, _)) => {
    //             println!("receive from remote: {:?}", &buf[0..recv_len]);
    //             source_write.write_all(&buf[0..recv_len]).await.expect("TODO: panic message");
    //         }
    //         Err(e) => {
    //             println!("Error receiving data from remote: {:?}", e);
    //             break;
    //         }
    //     }
    //     println!("finished");

    // }

    println!("udp");


}