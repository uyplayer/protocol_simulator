/*
 * @Author: uyplayer
 * @Date: 2023/9/7 13:39
 * @Email: uyplayer@qq.com
 * @File: client
 * @Software: CLion
 * @Dir: proxy_simulator / src
 * @Project_Name: proxy_simulator
 * @Description:    client side
 */



use std::string::ToString;
use std::sync::Arc;
use clap::{Arg, Command};
use proxy_simulator::helper::daemonize::set_daemon;
use proxy_simulator::selector::client_run::client_run;
use clap::builder::TypedValueParser;
use proxy_simulator::helper::logger;
use proxy_simulator::helper::config;
///
///  client side should helper different protocols base on package contents
///  and send them protocal handler to manipulate them
/// 1. http protocal
/// 2. https protocal
/// 3. socks5 protocal
///

#[tokio::main]
async fn main(){
    let matches = Command::new("client side")
        .about("proxy simulation experiment")
        .version("0.0.1")
        .arg_required_else_help(false)
        .author("@uyplayer")
        .arg( Arg::new("config")
                  .short('c')
                  .long("config")
                  .default_value("")
                  .required(false)
                  .help("client config file path")).get_matches();
    let one = matches.get_one::<String>("config").map(|c|{
      c.to_string()
    });
    let config_toml = match one{
        Some(path) => {
            if path == ""{
                config::parse_toml("src/config/client.toml".to_string())
            }else {
                config::parse_toml(path)
            }
        },
        None => {
            unimplemented!();
        },
    };
    let daemonize= config_toml["daemon"].parse::<bool>().unwrap();
    if daemonize {
        let name = "proxy simulator client";
        set_daemon(name);
    }
    logger::init_logger().await;
    log::info!("Client command initialized");
    log::info!("local port : {}",config_toml["local_port"]);
    if config_toml["remote_addr"] == ""{
        log::info!("destination host address  {}", "not set");
    }else {
        log::info!("remote host address : {}", config_toml["remote_addr"]);
    }
    // start client
    client_run(config_toml).await;


}


