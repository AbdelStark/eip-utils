use clap::{load_yaml, App};
use std::process;
mod cli;
mod util;

fn main() {
    let yaml = load_yaml!("../resources/cli.yaml");
    let matches = App::from(yaml).get_matches();
    if let Some(ref matches) = matches.subcommand_matches("eip1559-compute-basefee") {
        // "$ eip-utils eip1559-compute-basefee" was run
        cli::eip1559_compute_base_fee(&matches);
        process::exit(0);
    }
    if let Some(_) = matches.subcommand_matches("eip1559-resources") {
        // "$ eip-utils tx-encode" was run
        cli::eip1559_resources();
        process::exit(0);
    }
}
