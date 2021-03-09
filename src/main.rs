use eip_utils::eip1559;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fs};

const TEST_FILE_PATH: &str = "/Users/abdelhamid/Documents/GitHub/rust/eip-utils/basefee-test.json";

fn main() {
    let data = fs::read_to_string(TEST_FILE_PATH);
    let data = match data {
        Ok(c) => c,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let test_cases = json_to_testcase(&data);
    for case in &test_cases {
        let parent_base_fee = BigUint::from(case.parent_base_fee);
        let parent_gas_used = BigUint::from(case.parent_gas_used);
        let parent_target_gas_used = BigUint::from(case.parent_target_gas_used);
        let base_fee =
            eip1559::compute_base_fee(&parent_base_fee, &parent_gas_used, &parent_target_gas_used);
        if base_fee.cmp(&BigUint::from(case.expected_base_fee)) != Ordering::Equal {
            panic!(
                "invalid base fee: expected {} - got {}",
                case.expected_base_fee, base_fee
            );
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TestCase {
    #[serde(rename = "parentBaseFee")]
    parent_base_fee: u64,
    #[serde(rename = "parentGasUsed")]
    parent_gas_used: u64,
    #[serde(rename = "parentTargetGasUsed")]
    parent_target_gas_used: u64,
    #[serde(rename = "expectedBaseFee")]
    expected_base_fee: u64,
}

fn json_to_testcase(data: &str) -> Vec<TestCase> {
    let t = serde_json::from_str(data);
    match t {
        Ok(c) => c,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}
