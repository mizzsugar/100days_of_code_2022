use anyhow::{self, Context};
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("invalid argument error: {0}")]
    InvalidArgumentError(String),
}

struct HitPoint {
    value: u16,
}

impl HitPoint {
    const MAX: u16 = 999;

    pub fn new(value: u16) -> Result<Self, AppError> {
        if Self::MAX < value {
            return Err(AppError::InvalidArgumentError(format!(
                "{}以下を指定してください",
                Self::MAX
            )));
        }
        Ok(HitPoint { value: value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_hit_point() {
        let _hit_point_1 = HitPoint::new(0);
        assert!(_hit_point_1.is_ok());
        assert_eq!(_hit_point_1.ok().unwrap().value, 0);

        let _hit_point_2 = HitPoint::new(1);
        assert!(_hit_point_2.is_ok());
        assert_eq!(_hit_point_2.ok().unwrap().value, 1);

        let _hit_point_3 = HitPoint::new(999);
        assert!(_hit_point_3.is_ok());
        assert_eq!(_hit_point_3.ok().unwrap().value, 999);
    }

    #[test]
    fn over_max_hit() {
        let _hit_point_3 = HitPoint::new(1000);
        // TODO: 自作した例外のテストが発生しているかのテストを書く
        // https://github.com/mizzsugar/100days_of_code_2022/issues/2
        assert!(_hit_point_3.is_err());
    }
}
