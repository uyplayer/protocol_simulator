/*
 * @Author: uyplayer
 * @Date: 2023/9/13 10:13
 * @Email: uyplayer@qq.com
 * @File: config
 * @Software: CLion
 * @Dir: proxy_simulator / src/helper
 * @Project_Name: proxy_simulator
 * @Description:
 */


use std::fs::File;
use std::io::Read;
use toml::Value;
use std::collections::HashMap;

// parse config toml file
pub fn parse_toml(path:String) -> HashMap<&'static str, String> {
    // save them  under hashmap
    let mut config :HashMap<&str,String>= HashMap::new();
    let mut file_open = File::open(path).expect("Unable to open file");
    let mut toml_str = String::new();
    file_open.read_to_string(&mut toml_str).expect("Unable to read file");
    // parsing
    let parsed_config: Value = toml::from_str(&toml_str).expect("Unable to parse TOML");

    let name = &parsed_config
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("Unnamed");
    config.insert("name",name.to_string());

    let protocol_type = &parsed_config
        .get("protocol_type")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown Protocol");
    config.insert("protocol_type",protocol_type.to_string());


    let local_config = &parsed_config
        .get("local")
        .and_then(|v| v.as_table())
        .unwrap();
    let local_addr = local_config
        .get("local_addr")
        .and_then(|v| v.as_str())
        .unwrap_or("127.0.0.1");
    config.insert("local_addr",local_addr.to_string());
    let local_port = local_config
        .get("local_port")
        .and_then(|v| v.as_str())
        .unwrap_or("1080");
    config.insert("local_port",local_port.to_string());


    let remote_config = &parsed_config
        .get("remote")
        .and_then(|v| v.as_table())
        .unwrap();

    let remote_addr = &remote_config
        .get("remote_addr")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    config.insert("remote_addr",remote_addr.to_string());
    let remote_port = &remote_config
        .get("remote_port")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    config.insert("remote_port",remote_port.to_string());

    let daemon_config = &parsed_config
        .get("daemon_linux")
        .and_then(|v| v.as_table())
        .unwrap();
    let daemon = daemon_config
        .get("daemon")
        .and_then(|v| v.as_str())
        .unwrap_or("false");
    config.insert("daemon",daemon.to_string());


    let trojan_config = &parsed_config
        .get("trojan")
        .and_then(|v| v.as_table())
        .unwrap();
    let trojan_passwd = trojan_config
        .get("password")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    config.insert("trojan_passwd",trojan_passwd.to_string());

    config
}


#[test]
fn test_config(){
   let path = "src/config/client.toml";
    let hash = parse_toml(path);
    println!("{:?}",hash);
    assert_eq!(0,0);
}


