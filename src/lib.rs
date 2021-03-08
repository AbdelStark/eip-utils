pub mod eip1559{
    
    pub fn new() -> Service{
        Service{}
    }

    pub struct Service{

    }

    impl Service {
        pub fn compute_base_fee(&self, parent_base_fee: &u64, parent_gas_used: &u64, parent_target_gas_used: &u64) -> u64{
            0
        }
    }

}