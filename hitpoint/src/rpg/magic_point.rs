use std::cmp;

struct MagicPoint {
    current_amount: i16,
    original_max_amout: i16,
    max_increments: Vec<i16>,
}

impl MagicPoint {
    const MIN: i16 = 0;

    pub fn current(self) -> i16 {
        self.current_amount
    }

    pub fn max(self) -> i16 {
        let mut amout = self.original_max_amout;
        for point in self.max_increments {
            amout += point;
        }
        amout
    }

    pub fn recover(self, recovery_amount: i16) -> i16 {
        cmp::min(self.current_amount + recovery_amount, self.max())
    }

    pub fn consume(self, consume_amout: i16) -> i16 {
        cmp::max(self.current_amount - consume_amout, Self::MIN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_magic_point() {
        let magic_point = MagicPoint {
            current_amount: 10,
            original_max_amout: 100,
            max_increments: vec![1, 2, 3],
        };
        let max_point = magic_point.max();
        let expected = 106;

        assert_eq!(max_point, expected);
    }

    #[test]
    fn test_recover_magic_point() {
        let magic_point = MagicPoint {
            current_amount: 10,
            original_max_amout: 100,
            max_increments: vec![1, 2, 3],
        };
        let recovered = magic_point.recover(95);
        let expected = 105;

        assert_eq!(recovered, expected);
    }

    #[test]
    fn test_recover_magic_point_over_max_point() {
        let magic_point = MagicPoint {
            current_amount: 10,
            original_max_amout: 100,
            max_increments: vec![1, 2, 3],
        };
        let recovered = magic_point.recover(97);
        let expected = 106;

        assert_eq!(recovered, expected);
    }

    #[test]
    fn test_consume_magic_point() {
        let magic_point = MagicPoint {
            current_amount: 10,
            original_max_amout: 100,
            max_increments: vec![1, 2, 3],
        };
        let recovered = magic_point.consume(9);
        let expected = 1;

        assert_eq!(recovered, expected);
    }

    #[test]
    fn test_consume_magic_point_under_min_point() {
        let magic_point = MagicPoint {
            current_amount: 10,
            original_max_amout: 100,
            max_increments: vec![1, 2, 3],
        };
        let recovered = magic_point.consume(11);
        let expected = 0;

        assert_eq!(recovered, expected);
    }
}
