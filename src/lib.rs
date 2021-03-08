pub mod eip1559{
    use num_bigint::BigUint;
    use std::cmp::Ordering;

    const BASEFEE_MAX_CHANGE_DENOMINATOR: u64  = 8;
    
    pub fn new() -> Service{
        Service{}
    }

    pub struct Service{

    }

    impl Service {
        pub fn compute_base_fee(&self, parent_base_fee: &BigUint, parent_gas_used: &BigUint, parent_target_gas_used: &BigUint) -> BigUint{
            match  parent_gas_used.cmp(&parent_target_gas_used) {
              Ordering::Equal => parent_base_fee.clone(),
              Ordering::Greater => self.compute_base_fee_increase(parent_base_fee, parent_gas_used, parent_target_gas_used),
              Ordering::Less => self.compute_base_fee_decrease(parent_base_fee, parent_gas_used, parent_target_gas_used)
            }
        }

        fn compute_base_fee_increase(&self, parent_base_fee: &BigUint, parent_gas_used: &BigUint, parent_target_gas_used: &BigUint) -> BigUint{
            let gas_delta = parent_gas_used - parent_target_gas_used;
            let fee_delta = (parent_base_fee * gas_delta) / parent_target_gas_used / BASEFEE_MAX_CHANGE_DENOMINATOR;
            parent_base_fee + fee_delta
        }

        fn compute_base_fee_decrease(&self, parent_base_fee: &BigUint, parent_gas_used: &BigUint, parent_target_gas_used: &BigUint) -> BigUint{
            let gas_delta = parent_target_gas_used - parent_gas_used;
            let fee_delta = (parent_base_fee * gas_delta) / parent_target_gas_used / BASEFEE_MAX_CHANGE_DENOMINATOR;
            parent_base_fee - fee_delta        
        }
    }

}