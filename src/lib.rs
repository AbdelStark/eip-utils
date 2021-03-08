pub mod eip1559{
    use num_bigint::BigUint;
    use std::cmp::Ordering;
    use num_traits::{Zero, One};

    
    pub fn new() -> Service{
        Service{}
    }

    pub struct Service{

    }

    impl Service {
        pub fn compute_base_fee(&self, parent_base_fee: &BigUint, parent_gas_used: &BigUint, parent_target_gas_used: &BigUint) -> BigUint{
            match  parent_gas_used.cmp(&parent_target_gas_used) {
              Ordering::Equal => parent_base_fee.clone(),
              Ordering::Greater => self.compute_base_fee_asc(parent_base_fee, parent_gas_used, parent_target_gas_used),
              Ordering::Less => self.compute_base_fee_desc(parent_base_fee, parent_gas_used, parent_target_gas_used)
            }
        }

        fn compute_base_fee_asc(&self, parent_base_fee: &BigUint, parent_gas_used: &BigUint, parent_target_gas_used: &BigUint) -> BigUint{
            Zero::zero()
        }

        fn compute_base_fee_desc(&self, parent_base_fee: &BigUint, parent_gas_used: &BigUint, parent_target_gas_used: &BigUint) -> BigUint{
            Zero::zero()
        }
    }

}