/// # Fee market change for ETH 1.0 chain
/// A transaction pricing mechanism that includes fixed-per-block network fee that is burned and dynamically expands/contracts block sizes to deal with transient congestion.
///
/// [Fee market change for ETH 1.0 chain](https://eips.ethereum.org/EIPS/eip-1559)
pub mod eip1559 {
    use num_bigint::BigUint;
    use num_traits::One;
    use std::cmp::{self, Ordering};

    const BASEFEE_MAX_CHANGE_DENOMINATOR: u64 = 8;

    ///
    /// Computes the base fee of a block given parent block header data.
    /// # Arguments
    /// * `parent_base_fee` - The value of the parent block base fee
    /// * `parent_gas_used` - The value of the parent block gas used
    /// * `parent_target_gas_used` - The value of the parent block target gas used
    ///
    /// # Examples
    ///
    /// ```
    /// use num_bigint::BigUint;
    /// use eip_utils::eip1559;
    /// let parent_base_fee = BigUint::from(1000000000 as u64);
    /// let parent_gas_used = BigUint::from(10000000 as u64);
    /// let parent_target_gas_used = BigUint::from(5000000 as u64);
    /// let base_fee = eip1559::compute_base_fee(&parent_base_fee, &parent_gas_used, &parent_target_gas_used);
    /// assert_eq!(base_fee, BigUint::from(1125000000 as u64));
    /// ```
    pub fn compute_base_fee(
        parent_base_fee: &BigUint,
        parent_gas_used: &BigUint,
        parent_target_gas_used: &BigUint,
    ) -> BigUint {
        match parent_gas_used.cmp(&parent_target_gas_used) {
            Ordering::Equal => parent_base_fee.clone(),
            Ordering::Greater => {
                compute_base_fee_increase(parent_base_fee, parent_gas_used, parent_target_gas_used)
            }
            Ordering::Less => {
                compute_base_fee_decrease(parent_base_fee, parent_gas_used, parent_target_gas_used)
            }
        }
    }

    fn compute_base_fee_increase(
        parent_base_fee: &BigUint,
        parent_gas_used: &BigUint,
        parent_target_gas_used: &BigUint,
    ) -> BigUint {
        let gas_delta = parent_gas_used - parent_target_gas_used;
        let fee_delta = cmp::max(
            (parent_base_fee * gas_delta) / parent_target_gas_used / BASEFEE_MAX_CHANGE_DENOMINATOR,
            One::one(),
        );
        parent_base_fee + fee_delta
    }

    fn compute_base_fee_decrease(
        parent_base_fee: &BigUint,
        parent_gas_used: &BigUint,
        parent_target_gas_used: &BigUint,
    ) -> BigUint {
        let gas_delta = parent_target_gas_used - parent_gas_used;
        let fee_delta =
            (parent_base_fee * gas_delta) / parent_target_gas_used / BASEFEE_MAX_CHANGE_DENOMINATOR;
        parent_base_fee - fee_delta
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;
    use serde::{Deserialize, Serialize};
    use std::{cmp::Ordering, fs, path::PathBuf};

    #[test]
    fn assert_basefee_computation_works() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test/eip1559/basefee/reference-test.json");
        let path = d.as_path().to_str().unwrap();
        let data = fs::read_to_string(path);
        let data = match data {
            Ok(c) => c,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };

        let test_cases = json_to_testcase(&data);
        for case in &test_cases {
            let parent_base_fee = BigUint::from(case.parent_base_fee);
            let parent_gas_used = BigUint::from(case.parent_gas_used);
            let parent_target_gas_used = BigUint::from(case.parent_target_gas_used);
            let base_fee = eip1559::compute_base_fee(
                &parent_base_fee,
                &parent_gas_used,
                &parent_target_gas_used,
            );
            assert_eq!(
                true,
                base_fee.cmp(&BigUint::from(case.expected_base_fee)) == Ordering::Equal
            );
        }
    }

    pub fn json_to_testcase(data: &str) -> Vec<TestCase> {
        let t = serde_json::from_str(data);
        match t {
            Ok(c) => c,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TestCase {
        #[serde(rename = "parentBaseFee")]
        pub parent_base_fee: u64,
        #[serde(rename = "parentGasUsed")]
        pub parent_gas_used: u64,
        #[serde(rename = "parentTargetGasUsed")]
        pub parent_target_gas_used: u64,
        #[serde(rename = "expectedBaseFee")]
        pub expected_base_fee: u64,
    }
}
