use crate::messages::r17::TAC;

// For testing whether a given TAC matches an integer.
impl PartialEq<u32> for TAC {
    fn eq(&self, other: &u32) -> bool {
        other < &(1 << 24) && self.0.as_slice() == &other.to_be_bytes().as_slice()[1..]
    }
}

#[cfg(test)]
mod tests {

    use super::TAC;

    #[test]
    fn compare_ta_to_u32() {
        struct TestCase {
            lhs: TAC,
            rhs: u32,
            result: bool,
        }

        let testcases = vec![TestCase {
            lhs: TAC(vec![00, 00, 01]),
            rhs: 01_u32,
            result: true,
        }];

        for tc in testcases {
            assert_eq!(tc.lhs == tc.rhs, tc.result);
        }
    }
}
