use eip_utils::eip1559;
use num_bigint::BigUint;


fn main() {
    let eip1559_service = &eip1559::new();
    let parent_base_fee: u64 = 1049238967;
    let parent_gas_used: u64  = 10000000;
    let parent_target_gas_used: u64 = 9000000;
    let parent_base_fee = BigUint::from(parent_base_fee);
    let parent_gas_used  = BigUint::from(parent_gas_used);
    let parent_target_gas_used = BigUint::from(parent_target_gas_used);
    let base_fee = eip1559_service.compute_base_fee(&parent_base_fee, &parent_gas_used, &parent_target_gas_used);
    println!("base fee: {}", &base_fee);
}
