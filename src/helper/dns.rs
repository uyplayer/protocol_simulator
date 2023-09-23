/*
 * @Author: uyplayer
 * @Date: 2023/9/12 23:08
 * @Email: uyplayer@qq.com
 * @File: dns
 * @Software: CLion
 * @Dir: proxy_simulator / src/helper
 * @Project_Name: proxy_simulator
 * @Description:
 */


use std::net::{Ipv4Addr, Ipv6Addr};
use dns_lookup::{lookup_host,lookup_addr};

pub enum AddressType {
    IPv4,
    IPv6,
    Domain,
    Invalid,
}

pub fn check_address_type(input: String) -> AddressType {
    if input.parse::<Ipv4Addr>().is_ok() {
        AddressType::IPv4
    } else if input.parse::<Ipv6Addr>().is_ok() {
        AddressType::IPv6
    } else {
        // Use a regular expression to check if the input is a valid domain name.
        // This is a simplified regex, and you can adjust it to your needs.
        let domain_regex = regex::Regex::new(r"^[a-zA-Z0-9.-]+$").unwrap();
        if domain_regex.is_match(&input) {
            AddressType::Domain
        } else {
            AddressType::Invalid
        }
    }
}


pub fn domain_to_ip(){

}

pub fn ip_to_domain(ip:String)->String{
    let  find_ip  = ip.split(":").collect::<Vec<_>>()[0];
    let name = lookup_host(find_ip).unwrap()[0];
    name.to_string()
}


