use clap::{load_yaml, App};
use std::process;
mod cli;
mod util;

fn main() {
    let yaml = load_yaml!("../resources/cli.yaml");
    let matches = App::from(yaml).get_matches();
    let eip_1559 = cli::EIP1559Cmd {};
    if let Some(ref matches) = matches.subcommand_matches("eip1559-compute-basefee") {
        // "$ eip-utils eip1559-compute-basefee" was run
        eip_1559.compute_base_fee(&matches);
        process::exit(0);
    }
    if let Some(_) = matches.subcommand_matches("eip1559-resources") {
        // "$ eip-utils tx-encode" was run
        eip_1559.resources();
        process::exit(0);
    }
    if let Some(_) = matches.subcommand_matches("tx-encode") {
        // "$ eip-utils tx-encode" was run
        tx_encode();
        process::exit(0);
    }
    println!("{}", BANNER);
}

fn tx_encode() {
    println!("encode tx");
    // https://crates.io/crates/rlp
    // https://github.com/rust-blockchain/ethereum
}

const BANNER: &str = r#"
#▓█████  ██▓ ██▓███      █    ██ ▄▄▄█████▓ ██▓ ██▓      ██████ 
▓█   ▀ ▓██▒▓██░  ██▒    ██  ▓██▒▓  ██▒ ▓▒▓██▒▓██▒    ▒██    ▒ 
▒███   ▒██▒▓██░ ██▓▒   ▓██  ▒██░▒ ▓██░ ▒░▒██▒▒██░    ░ ▓██▄   
▒▓█  ▄ ░██░▒██▄█▓▒ ▒   ▓▓█  ░██░░ ▓██▓ ░ ░██░▒██░      ▒   ██▒
░▒████▒░██░▒██▒ ░  ░   ▒▒█████▓   ▒██▒ ░ ░██░░██████▒▒██████▒▒
░░ ▒░ ░░▓  ▒▓▒░ ░  ░   ░▒▓▒ ▒ ▒   ▒ ░░   ░▓  ░ ▒░▓  ░▒ ▒▓▒ ▒ ░
 ░ ░  ░ ▒ ░░▒ ░        ░░▒░ ░ ░     ░     ▒ ░░ ░ ▒  ░░ ░▒  ░ ░
   ░    ▒ ░░░           ░░░ ░ ░   ░       ▒ ░  ░ ░   ░  ░  ░  
   ░  ░ ░                 ░               ░      ░  ░      ░  
                                                              
"#;
