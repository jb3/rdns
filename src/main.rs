mod commands;
mod dns;

use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let matches = App::new("RDNS")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author("Joe Banks <joe@jb3.dev>")
        .settings(&[
            AppSettings::ArgRequiredElseHelp,
            AppSettings::GlobalVersion,
            AppSettings::InferSubcommands,
        ])
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("ADDRESS")
                .help("Address (v4 or v6) to perform an RDNS lookup on")
                .required(false)
                .index(1),
        )
        .subcommand(
            SubCommand::with_name("bulk")
                .about("Bulk RDNS lookups")
                .arg(
                    Arg::with_name("mode")
                        .required(true)
                        .short("m")
                        .takes_value(true)
                        .help("Mode to use for bulk lookup")
                        .possible_values(&["raw", "cidr"]),
                )
                .arg(
                    Arg::with_name("SOURCE")
                        .required(true)
                        .index(1)
                        .help("The source of address information, in the format specified by -m."),
                ),
        )
        .get_matches();

    if let Some(address) = matches.value_of("ADDRESS") {
        commands::single::run_single(address)
    }

    if let Some(sub_matches) = matches.subcommand_matches("bulk") {
        commands::bulk::run_bulk(sub_matches)
    }
}
