/*
 * @Author: uyplayer
 * @Date: 2023/9/7 15:46
 * @Email: uyplayer@qq.com
 * @File: https
 * @Software: CLion
 * @Dir: proxy_simulator / src/protocols/https
 * @Project_Name: proxy_simulator
 * @Description:
 */


use std::collections::HashMap;
use std::io::{BufRead};
use tokio::io::{AsyncReadExt, AsyncWriteExt, copy_bidirectional};
use tokio::net::TcpStream;
use crate::helper::protocol_info::{PackageProtocol};


pub async fn https_handler(mut stream: TcpStream, protocol: PackageProtocol, config: HashMap<&str, String>) {
    let (mut source_read, mut source_write) = stream.split();
    let host = protocol.destination_host.unwrap();
    let port = protocol.destination_port.unwrap();
    let dest_add = host.clone() + ":" + &*port;
    let mut buffer = [0; 1024];
    let len = source_read.read(&mut buffer).await.expect("peek failed");
    let request_str = String::from_utf8_lossy(&buffer[..len]);
    drop(buffer);
    drop(request_str);
    // without middle server
    if config["remote_addr"].is_empty() {
        let mut destination = match TcpStream::connect(&dest_add).await {
            Ok(des_stream) => {
                des_stream
            }
            Err(err) => {
                log::error!("Error connecting to destination : {:?}", err);
                return;
            }
        };
        // send client 200 response
        let response = "HTTP/1.1 200 Connection Established\r\n\r\n";
        if let Err(e) = source_write.write_all(response.as_ref()).await {
            log::error!("Error sending 200 response to client: {:?}", e);
            return;
        }
        // Use tokio::io::copy_bidirectional to handle data forwarding
        if let Err(e) = copy_bidirectional(&mut stream, &mut destination).await {
            log::error!("Error forwarding data between client and destination: {:?}", e);
        }
    } else {
        // Handle the case with a remote server (middle server) here
        // You can establish a connection to the middle server and implement your logic accordingly.
    }
}