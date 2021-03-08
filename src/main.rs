use eip_utils::eip1559;


fn main() {
    let eip1559_service = &eip1559::new();
    let parent_base_fee: u64 = 0;
    let parent_gas_used: u64 = 0;
    let parent_target_gas_used: u64 = 0;
    let base_fee = eip1559_service.compute_base_fee(&parent_base_fee, &parent_gas_used, &parent_target_gas_used);
    println!("base fee: {}", &base_fee);
}
