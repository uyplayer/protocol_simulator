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



use std::ops::Add;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use crate::helper::protocol_info::{extract_protocol_info, Protocol, ProtocolsList};
use crate::protocols::http::http;
use crate::protocols::https::https;

#[derive(Debug,Clone)]
pub struct Config {
    /// local port to listen application data
    pub local_port: String,
    /// client send data to destination host address
    /// if dest_host is set , dest_host address will be middle server
    /// if dest_host not set , data wil be directly sent it's original destination
    pub dest_host: String,

}
pub async fn  client_run(config:Config) {
    let addr = "127.0.0.1".to_string();
    let local_add = addr.add(":").add(&*(config.clone()).local_port);
    let listener = TcpListener::bind(&local_add).await.unwrap();
    log::info!("client start at {} listening  package coming from application",&local_add);
    loop {
        match listener.accept().await {
            Ok((socket, socket_add)) => {
                let config_clone = config.clone(); // Clone the config
                let task = tokio::spawn(async move {
                    log::info!("new application connected at : {}", socket_add);
                    select_protocal(socket,config_clone).await;
                });

            }
            Err(err) => {
                log::error!("Error accepting connection: {:?}", err);
            }
        }
    }
}
async fn select_protocal (mut stream:TcpStream, config:Config){
    // extract protocol info from package
    let protocal = extract_protocol_info(&stream).await;
    let pro_info = protocal.clone();
    match pro_info.protocal_type {
        Some(ProtocolsList::HTTP) | Some(ProtocolsList::HTTPS) | Some(ProtocolsList::SOCKS5) => {
            log::info!("{:?} Protocol , destination at : {}:{}", pro_info.protocal_type.clone().unwrap(), pro_info.destination_host.unwrap(), pro_info.destination_port.unwrap());
        },
        _ => {
            log::error!("Unrecognized Protocol");
            if let Err(err) = stream.shutdown().await {
                log::error!("Failed to close connection: {:?}", err);
            }
            log::error!("close connection because of unrecognized protocal");
            return;

        }
    }
    match protocal.protocal_type{
        Some(ProtocolsList::HTTP)=>{
            http::http_handler(stream, protocal, config).await;
        },
        Some(ProtocolsList::HTTPS) =>{
            https::https_handler(stream, protocal, config).await;
        } ,
        Some(ProtocolsList::SOCKS5)=>{

        },
        _ => {
            log::error!("Unrecognized Protocol , handler not implement");
            if let Err(err) = stream.shutdown().await {
                log::error!("Failed to close connection: {:?}", err);
            }
        }
    }


}
