use crate::dns;
use clap::ArgMatches;

use colored::*;
use dialoguer::Confirm;
use ipnetwork::{IpNetwork, NetworkSize};
use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use trust_dns_client::client::SyncClient;
use trust_dns_client::udp::UdpClientConnection;

const MAX_SAFE: u128 = 65_536;

pub fn run_bulk(matches: &ArgMatches) {
    let ips = match matches.value_of("mode") {
        Some("cidr") => from_cidr(matches.value_of("SOURCE").unwrap()),
        Some("raw") => unimplemented!(),
        _ => unimplemented!(),
    };

    let ips_iter = match ips {
        Some(ips) => ips,
        None => return,
    };

    let ips_vec: Vec<_> = ips_iter.collect();

    ips_vec
        .into_par_iter()
        .for_each_init(|| dns::create_client(), |client, x| lookup_addr(client, x));
}

fn lookup_addr(client: &mut SyncClient<UdpClientConnection>, ip: IpAddr) {
    let domain = dns::generate_ptr_domain(ip).unwrap();

    let ptr = dns::get_ptr(&domain, &client);

    if let Some(rec) = ptr {
        println!("{} resolves to {}", format!("{:?}", ip).cyan(), rec.cyan())
    }
}

fn from_cidr(cidr: &str) -> Option<Box<dyn Iterator<Item = IpAddr>>> {
    let ip_frag = cidr.split_once('/');

    if let Some((ip, len)) = ip_frag {
        let ip: Result<IpAddr, _> = ip.parse();

        let len = len.parse::<u8>();

        let cidr_len = if let Ok(length) = len {
            length
        } else {
            println!("{}", "Could not parse CIDR size".red());
            return None;
        };

        let cidr = if let Ok(ip_addr) = ip {
            match ip_addr {
                IpAddr::V4(v4_addr) => process_v4_cidr(v4_addr, cidr_len),
                IpAddr::V6(v6_addr) => process_v6_cidr(v6_addr, cidr_len),
            }
        } else {
            println!("{}", "Could not parse IP out of CIDR".red());
            None
        };

        match cidr {
            Some(ip_range) => get_addresses(ip_range),
            None => None,
        }
    } else {
        None
    }
}

fn process_v6_cidr(ip: Ipv6Addr, len: u8) -> Option<IpNetwork> {
    if len > 128 {
        println!("{}", "Invalid CIDR size for IPv6".red());
        return None;
    }

    if let Ok(cidr) = IpNetwork::new(IpAddr::V6(ip), len) {
        Some(cidr)
    } else {
        println!("{}", "Could not create IPv6 CIDR".red());
        None
    }
}

fn process_v4_cidr(ip: Ipv4Addr, len: u8) -> Option<IpNetwork> {
    if len > 32 {
        println!("{}", "Invalid CIDR size for IPv4".red());
        return None;
    }

    if let Ok(cidr) = IpNetwork::new(IpAddr::V4(ip), len) {
        Some(cidr)
    } else {
        println!("{}", "Could not create IPv4 CIDR".red());
        None
    }
}

fn get_addresses(cidr: IpNetwork) -> Option<Box<dyn Iterator<Item = IpAddr>>> {
    let cidr_size_raw = cidr.size();

    let cidr_size = match cidr_size_raw {
        NetworkSize::V4(size) => size as u128,
        NetworkSize::V6(size) => size,
    };

    if cidr_size > MAX_SAFE {
        if !Confirm::new()
            .with_prompt(&format!(
                "This query will target over {} IPs, are you sure you want to run it?",
                cidr_size.to_formatted_string(&Locale::en).red()
            ))
            .interact()
            .ok()?
        {
            println!("{}", "Aborting.".red());
            return None;
        }
    }

    Some(Box::new(cidr.iter()))
}
