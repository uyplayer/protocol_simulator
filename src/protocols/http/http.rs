/*
 * @Author: uyplayer
 * @Date: 2023/9/7 15:38
 * @Email: uyplayer@qq.com
 * @File: http
 * @Software: CLion
 * @Dir: proxy_simulator / src/protocols/http
 * @Project_Name: proxy_simulator
 * @Description:
 */


use std::collections::HashMap;
use tokio::io::copy_bidirectional;
use tokio::net::TcpStream;
use crate::selector::client_run;
use crate::helper::protocol_info::{PackageProtocol};

pub async fn http_handler(mut stream: TcpStream, protocol: PackageProtocol, config: HashMap<&str, String>) {
    let (mut source_read, mut source_write) = stream.split();
    let host = protocol.destination_host.unwrap();
    let port = protocol.destination_port.unwrap();
    let dest_add = host.clone() + ":" + &*port;
    if config["remote_addr"].is_empty() {
        let mut destination = TcpStream::connect(&dest_add).await.unwrap();
        if let Err(e) = copy_bidirectional(&mut stream, &mut destination).await {
            log::error!("Error forwarding data between client and destination: {:?}", e);
        }
    } else {
        // Handle the case with a remote server (middle server) here
        // You can establish a connection to the middle server and implement your logic accordingly.
    }
}
