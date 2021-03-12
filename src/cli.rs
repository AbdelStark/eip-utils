use crate::util;
use clap::ArgMatches;
use eip_utils::eip1559;

pub struct EIP1559Cmd {}

impl EIP1559Cmd {
    pub fn compute_base_fee(&self, matches: &ArgMatches) {
        let parent_base_fee = util::option_to_big(matches.value_of("parent-base-fee"));
        let parent_gas_used = util::option_to_big(matches.value_of("parent-gas-used"));
        let parent_target_gas_used =
            util::option_to_big(matches.value_of("parent-target-gas-used"));
        let base_fee =
            eip1559::compute_base_fee(&parent_base_fee, &parent_gas_used, &parent_target_gas_used);
        println!("{}", base_fee);
    }

    pub fn resources(&self) {
        println!("https://hackmd.io/@timbeiko/1559-resources");
    }
}
