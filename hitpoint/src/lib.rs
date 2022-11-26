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
        // TODO: 自作のInvalidArgumentErrorを定義し、それを発生させる
        // Rust<Self, InvalidArgumentError>をnewの返り値とする
        // https://github.com/mizzsugar/100days_of_code_2022/issues/1
        if Self::MAX < value {
            return Err(AppError::InvalidArgumentError(format!("{}以下を指定してください", Self::MAX)))
        }
        Ok(HitPoint { value: value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_hit_point() {
        // TODO: テストコード修正
        let _hit_point_1 = HitPoint::new(0);
        assert_eq!(_hit_point_1.value, 0);

        // let _hit_point_2 = HitPoint::new(1);
        // assert_eq!(_hit_point_2.value, 1);

        // let _hit_point_3 = HitPoint::new(999);
        // assert_eq!(_hit_point_3.value, 999);
    }

    #[test]
    #[should_panic(expected = "999以下を指定してください")]
    fn over_max_hit() {
        let hit_point_3 = HitPoint::new(1000);
    }
}
