use std::fs;
use eip_utils::eip1559;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

const TEST_FILE_PATH: &str = "/Users/abdelhamid/Documents/GitHub/rust/eip-utils/basefee-test.json";

fn main() {
    let eip1559_service = &eip1559::new();
    let parent_base_fee: u64 = 1000000000;
    let parent_gas_used: u64  = 7000000;
    let parent_target_gas_used: u64 = 6000000;
    let parent_base_fee = BigUint::from(parent_base_fee);
    let parent_gas_used  = BigUint::from(parent_gas_used);
    let parent_target_gas_used = BigUint::from(parent_target_gas_used);
    let base_fee = eip1559_service.compute_base_fee(&parent_base_fee, &parent_gas_used, &parent_target_gas_used);
    println!("base fee: {}", &base_fee);

    let data= fs::read_to_string(TEST_FILE_PATH);
    let data = match data {
        Ok(c) => c,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let test_case = json_to_testcase(&data);
    println!("test case: {:?}", test_case);
}


#[derive(Debug, Serialize, Deserialize)]
struct TestCase{
    parent_base_fee: u64,
    parent_gas_used: u64,
    parent_target_gas_used: u64
}

fn json_to_testcase(data: &str) -> Vec<TestCase>{
    let t = serde_json::from_str(data);
    match t {
        Ok(c) => c,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}