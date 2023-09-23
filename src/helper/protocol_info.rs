/*
 * @Author: uyplayer
 * @Date: 2023/9/7 13:57
 * @Email: uyplayer@qq.com
 * @File: main
 * @Software: CLion
 * @Dir: proxy_simulator / src/helper
 * @Project_Name: proxy_simulator
 * @Description:
 */



use std::collections::HashMap;
use std::fmt::{Display};
use tokio::net::{TcpStream};



///
///
///        supporting protocols list
/// =========================================
///          http / https / socks5 / trojan
/// =========================================
///
///
#[derive(Debug,PartialEq,Clone)]
pub enum ProtocolsList {
    HTTP,
    HTTPS,
    SOCKS5,
    TROJAN,
    UDP
}


///
/// save protocal information
///
#[derive(Debug,Clone)]
pub struct PackageProtocol {
    pub protocal_type:Option<ProtocolsList>,
    pub destination_host:Option<String>,
    pub destination_port : Option<String>,
    pub source_ip:Option<String>,
    pub source_port:Option<String>,
}


pub async fn extract_protocol_info(stream: &TcpStream, config: HashMap<&str, String>) ->PackageProtocol
{
    let mut protocal = PackageProtocol {
        protocal_type: None,
        destination_host: None,
        destination_port: None,
        source_ip: None,
        source_port: None,
    };
    let mut buffer = [0; 1024];
    let len = stream.peek(&mut buffer).await.expect("peek failed");
    let mut buffer = &buffer[..len];
    let request_str = String::from_utf8_lossy(&buffer);
    // http https
    if request_str.contains("HTTP") && !request_str.is_empty(){
        let host_line = request_str.lines().find(|line| line.contains("Host:"));
        let Some(line) = host_line else {unimplemented!()};
        let split:Vec<_> = line.split(":").collect();
        let host = split[1].trim();
        let port = match split.len() {
            2 => Some("80".to_string()),
            _ => Some(split[2].to_string()),
        };
        let source:Vec<_> = stream.peer_addr().unwrap().to_string().split(":").map(|x|{
            x.to_string()
        }).collect();

        match request_str.contains("CONNECT") {
            true => {
                if config["protocol_type"] == "socks5" {
                    protocal.protocal_type = Some(ProtocolsList::SOCKS5);
                } else if config["protocol_type"] == "trojan" {
                    protocal.protocal_type = Some(ProtocolsList::TROJAN);
                } else if config["protocol_type"] == "udp" {
                    protocal.protocal_type = Some(ProtocolsList::UDP);
                }
                else {
                    protocal.protocal_type = Some(ProtocolsList::HTTPS);
                }

            }
            _ => {
                protocal.protocal_type = Some(ProtocolsList::HTTP);
            }
        }
        protocal.destination_host = Option::from(host.to_string());
        protocal.destination_port = port;
        protocal.source_ip =Some(source[0].to_string());
        protocal.source_port =Some(source[1].to_string());


    }
    protocal

}



