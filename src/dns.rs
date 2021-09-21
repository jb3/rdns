use std::str::FromStr;
use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns_client::udp::UdpClientConnection;

use std::net::IpAddr;

const MAX_ATTEMPTS: usize = 3;

pub fn create_client() -> SyncClient<UdpClientConnection> {
    let address = "8.8.8.8:53".parse().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();

    SyncClient::new(conn)
}

pub fn generate_ptr_domain_string(address: &str) -> Option<String> {
    let ip_parsed = address.parse();

    if let Ok(ip) = ip_parsed {
        generate_ptr_domain(ip)
    } else {
        None
    }
}

pub fn generate_ptr_domain(address: IpAddr) -> Option<String> {
    if let IpAddr::V4(ip) = address {
        let octets = ip.octets();

        let formatted_ptr = format!(
            "{}.{}.{}.{}.in-addr.arpa.",
            octets[3], octets[2], octets[1], octets[0]
        );

        Some(formatted_ptr)
    } else if let IpAddr::V6(ip) = address {
        let octets = ip.octets();

        let mut rdns_data: Vec<char> = Vec::new();

        for octet in octets {
            let hex = format!("{:2x}", octet);
            let mut chars = hex.chars();
            rdns_data.push(chars.nth(0).unwrap().clone());
            rdns_data.push(chars.nth(0).unwrap().clone());
        }

        rdns_data.reverse();

        let mut fqdn = String::new();

        for char in rdns_data {
            if char == ' ' {
                fqdn.push('0')
            } else {
                fqdn.push(char)
            }

            fqdn.push('.')
        }

        fqdn.push_str("ip6.arpa.");

        Some(String::from(fqdn))
    } else {
        println!("Could not parse IP.");
        None
    }
}

pub fn get_ptr(fqdn: &str, client: &SyncClient<UdpClientConnection>) -> Option<String> {
    let mut attempts = 0;

    while attempts <= MAX_ATTEMPTS {
        let name = Name::from_str(&fqdn).unwrap();

        let resp: Result<DnsResponse, _> = client.query(&name, DNSClass::IN, RecordType::PTR);

        if let Ok(response) = resp {
            let answers: &[Record] = response.answers();

            if answers.len() > 0 {
                if let &RData::PTR(ref name) = answers[0].rdata() {
                    return Some(name.to_ascii());
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            attempts += 1;
            continue;
        }
    }

    None
}
