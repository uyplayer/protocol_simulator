/*
 * @Author: uyplayer
 * @Date: 2023/9/7 15:46
 * @Email: uyplayer@qq.com
 * @File: socks5
 * @Software: CLion
 * @Dir: proxy_simulator / src/protocols/socks5
 * @Project_Name: proxy_simulator
 * @Description:
 */


use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt, copy_bidirectional};
use tokio::net::TcpStream;
use crate::helper::protocol_info::{PackageProtocol};


pub async fn socks5_handler(mut stream: TcpStream, protocol: PackageProtocol, config: HashMap<&str, String>) {
    let (mut source_read, mut source_write) = stream.split();
    let host = protocol.destination_host.unwrap();
    let port = protocol.destination_port.unwrap();
    let dest_add = host.clone() + ":" + &*port;

    /// need to clean up socket channel from client
    /// if not , this mean http connect package send to destination
    /// and cased failed to tsl handshake
    let mut buffer = [0; 1024];
    let len = source_read.read(&mut buffer).await.expect("peek failed");
    let request_str = String::from_utf8_lossy(&buffer[..len]);
    drop(buffer);
    drop(request_str);
    // stream to proxy server
    let mut destination = match TcpStream::connect(&config["remote_addr"]).await {
        Ok(des_stream) => {
            des_stream
        }
        Err(err) => {
            log::error!("Error connecting to destination : {:?}", err);
            return;
        }
    };
    // send version,methods,method
    destination.write_all(&[0x05, 0x01, 0x00]).await.expect("TODO: panic message");
    // read two bytes
    let mut response = [0u8; 2];
    destination.read_exact(&mut response).await.expect("TODO: panic message");
    if response == [0x05, 0x00] {
        let mut request = [0u8; 10];
        request[0] = 0x05;
        // connect
        request[1] = 0x01;
        // ip4
        request[3] = 0x01;
        request[4..8].copy_from_slice(host.as_ref());
        request[8..10].copy_from_slice(port.as_ref());
        // write
        destination.write_all(&request).await.expect("TODO: panic message");
        // read server response
        let mut response = [0u8; 10];
        destination.read_exact(&mut response).await.expect("TODO: panic message");
        if response[1] == 0x00 {
            log::info!("connected to proxy server");
            // send application 200 response
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
            log::error!("disconnecting all stream ,because  of failed to connect proxy server");
            drop(destination);
            drop(stream);
            return;
        }
    } else {
        panic!(" don't support authentication")
    }
}