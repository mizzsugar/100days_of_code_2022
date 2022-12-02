use anyhow::{self, Context};
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("invalid argument error: {0}")]
    InvalidArgumentError(String),
}

struct HitPoint {
    value: i16,
}

impl HitPoint {
    const MIN: i16 = 0;
    const MAX: i16 = 999;

    pub fn new(value: i16) -> Result<Self, AppError> {
        if Self::MAX < value {
            return Err(AppError::InvalidArgumentError(format!(
                "{}以下を指定してください",
                Self::MAX
            )));
        }
        Ok(HitPoint { value: value })
    }

    pub fn damage(&self, damage_amount: i16) -> Self {
        let damaged = self.value - damage_amount;
        let corrected = if damaged < Self::MIN {
            Self::MIN
        } else {
            damaged
        };
        Self { value: corrected }
    }

    pub fn recover(&self, recovery_amount: i16) -> Self {
        let recovered = self.value + recovery_amount;
        let corrected = if Self::MAX < recovered {
            Self::MAX
        } else {
            recovered
        };
        Self { value: corrected }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_hit_point() {
        let _hit_point_1 = HitPoint { value: 0 };
        assert_eq!(_hit_point_1.value, 0);

        let _hit_point_2 = HitPoint { value: 1 };
        assert_eq!(_hit_point_2.value, 1);

        let _hit_point_3 = HitPoint { value: 999 };
        assert_eq!(_hit_point_3.value, 999);
    }

    #[test]
    fn over_max_hit() {
        let _hit_point_3 = HitPoint::new(1000);
        // TODO: 自作した例外のテストが発生しているかのテストを書く
        // https://github.com/mizzsugar/100days_of_code_2022/issues/2
        assert!(_hit_point_3.is_err());
    }

    #[test]
    fn damage() {
        let _hit_point = HitPoint{ value: 100};
        let corrected = _hit_point.damage(99);
        assert_eq!(corrected.value, 1)
    }

    #[test]
    fn damage_over_current_point() {
        let _hit_point = HitPoint{ value: 100};
        let corrected = _hit_point.damage(101);
        assert_eq!(corrected.value, 0)
    }
}
