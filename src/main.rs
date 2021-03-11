use clap::{load_yaml, App, ArgMatches};
use eip_utils::eip1559;
use num_bigint::BigUint;
use std::{process::exit, str::FromStr};

fn main() {
    let yaml = load_yaml!("../resources/cli.yaml");
    let matches = App::from(yaml).get_matches();
    if let Some(ref matches) = matches.subcommand_matches("eip1559-compute-basefee") {
        // "$ eip-utils eip1559-compute-basefee" was run
        eip1559_compute_base_fee(&matches);
        exit(0);
    }
    println!("{}", BANNER);
}

fn eip1559_compute_base_fee(matches: &ArgMatches) {
    let parent_base_fee = option_to_big(matches.value_of("parent-base-fee"));
    let parent_gas_used = option_to_big(matches.value_of("parent-gas-used"));
    let parent_target_gas_used = option_to_big(matches.value_of("parent-target-gas-used"));
    let base_fee =
        eip1559::compute_base_fee(&parent_base_fee, &parent_gas_used, &parent_target_gas_used);
    println!("{}", base_fee);
}

fn option_to_big(value: Option<&str>) -> BigUint {
    str_to_big(value.unwrap())
}

fn str_to_big(value: &str) -> BigUint {
    BigUint::from_str(value).expect("invalid literal value specified, expected valid BigUint value")
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
