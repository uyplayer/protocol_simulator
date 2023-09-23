/*
 * @Author: uyplayer
 * @Date: 2023/9/12 16:34
 * @Email: uyplayer@qq.com
 * @File: rojan
 * @Software: CLion
 * @Dir: proxy_simulator / src/protocols/trojan
 * @Project_Name: proxy_simulator
 * @Description:
 */


use std::collections::HashMap;
use std::sync::Arc;
use rustls::{RootCertStore};
use tokio::net::TcpStream;
use crate::helper::protocol_info::PackageProtocol;
use crate::helper::dns::{check_address_type, AddressType, ip_to_domain};
use sha2::{Digest,Sha224};
use hex::encode;
use rand;
use rand::Rng;

pub async fn trojan_handler(mut stream: TcpStream, protocol: PackageProtocol, config: HashMap<&str, String>) {
    // make tls handshake
    let mut root_store = RootCertStore::empty();
    root_store.add_trust_anchors(
        webpki_roots::TLS_SERVER_ROOTS
            .iter()
            .map(|ta| {
                rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                    ta.subject,
                    ta.spki,
                    ta.name_constraints,
                )
            })
    );
    let config_tls = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let rc_config = Arc::new(config_tls);
    let host_name = config["remote_addr"].clone();
    let addr = match check_address_type(host_name.clone()) {
        AddressType::IPv4 | AddressType::IPv6 => {
            ip_to_domain(host_name.clone())
        }
        _ => {
            host_name
        }
    };
    let domain = addr.as_str().try_into().unwrap();
    let mut client = match rustls::ClientConnection::new(rc_config, domain) {
        Ok(client) => {
            client
        }
        Err(e) => {
            panic!("unable to connect server ");
        }
    };
    // +-----------------------+---------+----------------+---------+----------+
    // | hex(SHA224(password)) |  CRLF   | Trojan Request |  CRLF   | Payload  |
    // +-----------------------+---------+----------------+---------+----------+
    // |          56           | X'0D0A' |    Variable    | X'0D0A' | Variable |
    // +-----------------------+---------+----------------+---------+----------+
    //         Request
    // +-----+------+----------+----------+
    // | CMD | ATYP | DST.ADDR | DST.PORT |
    // +-----+------+----------+----------+
    // |  1  |  1   | Variable |    2     |
    // +-----+------+----------+----------+
    let password = &config["trojan_passwd"];
    let hash_result = encode_password(&password).await;
    let cmd:u8 = 0x01; // CONNECT
    let atyp :u8= 0x03; // DOMAINNAME
    let crlf = vec![0x0D, 0x0A];
    let port:u16 =  config["remote_port"].parse::<u16>().unwrap();
    let high_byte = (port >> 8) as u8;
    let low_byte = (port & 0xFF) as u8;
    let mut request = Vec::new();
    request.extend_from_slice(hash_result.as_bytes());
    request.extend(&crlf);
    request.push(cmd);
    request.push(atyp);
    request.push(config["remote_addr"].len() as u8);
    request.extend(config["remote_addr"].as_bytes());
    request.push(high_byte);
    request.push(low_byte);
    request.extend(&crlf);
    request.extend(payload().await.as_bytes());
    parse_request(request).await;

}
async fn encode_password(pw : &str)->String{
    let mut hasher = Sha224::new();//28
    hasher.update(pw);
    let result = hasher.finalize();
    let hex_result = encode(&result);
    hex_result
}
async fn payload()->String{

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

    let mut rng = rand::thread_rng();
    let payload_len = rng.gen_range(10..=100);

    let payload: String = (0..payload_len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    payload
}
async fn parse_request(request:Vec<u8>){
    let hash_result_hex: Vec<u8> = request[0..56].to_vec();
    let cmd: u8 = request[58];
    let atyp: u8 = request[59];
    let size = request[60] as usize;
    let dst_addr_start = 61usize;
    let dst_addr_end = dst_addr_start + size;
    let dst_addr: Vec<u8> = request[dst_addr_start..dst_addr_end].to_vec();
    let dst_port_start = dst_addr_end;
    let dst_port_end = dst_port_start + 2;
    let dst_port_bytes: [u8; 2] = [request[dst_port_start], request[dst_port_start + 1]];
    let dst_port = u16::from_be_bytes(dst_port_bytes);
    let payload_start = dst_port_end + 2;
    let payload: Vec<u8> = request[payload_start..].to_vec();
    println!("Password Hash: {:?}", hash_result_hex);
    println!("CMD: {}", cmd);
    println!("ATYP: {}", atyp);
    println!("DST.ADDR: {}", String::from_utf8_lossy(&dst_addr));
    println!("DST.PORT: {}", dst_port);
    println!("Payload: {:?}", payload);
}
