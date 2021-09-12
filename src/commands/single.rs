use crate::dns;
use colored::*;

pub fn run_single(address: &str) {
    let client = dns::create_client();

    let domain = dns::generate_ptr_domain_string(address);

    if let Some(ptr_name) = domain {
        let ptr = dns::get_ptr(&ptr_name, &client);

        if let Some(rec) = ptr {
            println!("{} resolves to {}", address.cyan(), rec.cyan())
        } else {
            let error = format!("No PTR record exists for {}", address.blue());

            println!("{}", error.red())
        }
    } else {
        println!("{}", "Could not parse IP".red());
        return;
    }
}
