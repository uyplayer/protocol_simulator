/*
 * @Author: uyplayer
 * @Date: 2023/9/7 16:49
 * @Email: uyplayer@qq.com
 * @File: run
 * @Software: CLion
 * @Dir: proxy_simulator / src/helper
 * @Project_Name: proxy_simulator
 * @Description:
 */



use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::net::SocketAddr;
use tokio::io::{AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::select;
use crate::helper::protocol_info::{extract_protocol_info, ProtocolsList};
use crate::protocols::http::http;
use crate::protocols::https::https;
use crate::protocols::socks5::socks5;
use crate::protocols::trojan::trojan;
use crate::protocols::udp::udp;


pub async fn  client_run(config:HashMap<&'static str,String>) {
    let bind_addr = format!("{}:{}",&config["local_addr"],&config["local_port"]);
    let listener = TcpListener::bind(&bind_addr).await.unwrap();
    log::info!("client start at {} listening  package coming from application",&bind_addr);
    loop {
        match listener.accept().await {
            Ok((socket, socket_add)) => {
                let config_clone = config.clone(); // Clone the config
                let task = tokio::spawn(async move {
                    log::info!("new application connected at : {}", socket_add);
                    select_protocal_tcp(socket,config_clone).await;
                });

            }
            Err(err) => {
                log::error!("Error accepting connection: {:?}", err);
            }
        }
    }
}

async fn select_protocal_tcp(mut stream: TcpStream, config: HashMap<&str, String>) {
    // extract protocol info from package
    let protocal = extract_protocol_info(&stream, config.clone()).await;
    let pro_info = protocal.clone();
    log::info!("config : {:?}",protocal);
    log::info!("config : {:?}",config);
    match pro_info.protocal_type {
        Some(ProtocolsList::HTTP) |
        Some(ProtocolsList::HTTPS) |
        Some(ProtocolsList::SOCKS5) |
        Some(ProtocolsList::TROJAN) |
        Some(ProtocolsList::UDP) => {
            log::info!("selected protocol:{:?}",pro_info.protocal_type.unwrap());
        }
        _ => {
            log::error!("Unrecognized Protocol");
            if let Err(err) = stream.shutdown().await {
                log::error!("Failed to close connection: {:?}", err);
            }
            log::error!("close connection because of unrecognized protocal");
            return;
        }
    }
    match protocal.protocal_type {
        Some(ProtocolsList::HTTP) => {
            http::http_handler(stream, protocal, config).await;
        }
        Some(ProtocolsList::HTTPS) => {
            https::https_handler(stream, protocal, config).await;
        }
        Some(ProtocolsList::UDP) => {
            udp::udp_handler(stream, protocal, config).await
        }
        Some(ProtocolsList::TROJAN) => {
            trojan::trojan_handler(stream, protocal, config).await
        }

        Some(ProtocolsList::SOCKS5) => {
            // if config.remote_addr == "" {
            //     log::error!("proxy server host address not set (host:port) in command params");
            //     exit(0);
            //
            // }
            socks5::socks5_handler(stream, protocal, config).await;
        }
        _ => {
            log::error!("Unrecognized Protocol , handler not implement");
            if let Err(err) = stream.shutdown().await {
                log::error!("Failed to close connection: {:?}", err);
            }
        }
    }
}
