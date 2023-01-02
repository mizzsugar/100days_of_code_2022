use std::cmp;
use std::fmt;
use crate::rpg::errors::AppError;

#[derive(Clone)]
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

    pub fn is_zero(&self) -> bool {
        self.value == Self::MIN
    }
}

#[derive(Clone)]
enum State {
    Normal,
    Dead,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::Normal => write!(f, "[State::Normal]"),
            State::Dead => write!(f, "[State::Dead]"),
        }
    }
}

#[derive(Clone)]
struct Member {
    hit_point: HitPoint,
    state: State,
}

impl Member {
    fn new(hp: HitPoint) -> Self {
        Self {
            hit_point: hp,
            state: State::Normal,
        }
    }
    fn damage(&mut self, damage_amount: i16) {
        let damaged = self.hit_point.damage(damage_amount);
        if damaged.is_zero() {
            self.state = State::Dead;
        }
    }
}

struct Party {
    members: Vec<Member>
}

impl Party {
    const MEX_MEMBER_COUNT: u16 = 4;

    pub fn new(members: Vec<Member>) -> Result<Self, AppError> {
        if usize::from(Self::MEX_MEMBER_COUNT) > members.len() {
            return Err(AppError::InvalidArgumentError(format!(
                "{}以下を指定してください",
                Self::MEX_MEMBER_COUNT
            )));
        }
        Ok(Party { members: members })
    }

    pub fn add(&self, new_member: Member) -> Result<Self, AppError> {
        if usize::from(Self::MEX_MEMBER_COUNT) == self.members.len() {
            return Err(AppError::InvalidArgumentError(format!(
                "パーティの定員である{}名に到達済です",
                Self::MEX_MEMBER_COUNT
            )));
        }
        let mut adding = self.members.to_vec();
        adding.push(new_member);
        Ok(Party { members: adding })
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
        let _hit_point = HitPoint { value: 100 };
        let corrected = _hit_point.damage(99);
        assert_eq!(corrected.value, 1)
    }

    #[test]
    fn damage_over_current_point() {
        let _hit_point = HitPoint { value: 100 };
        let corrected = _hit_point.damage(101);
        assert_eq!(corrected.value, 0)
    }

    #[test]
    fn recover() {
        let _hit_point = HitPoint { value: 1 };
        let corrected = _hit_point.recover(998);
        assert_eq!(corrected.value, 999)
    }

    #[test]
    fn recover_over_max_point() {
        let _hit_point = HitPoint { value: 999 };
        let corrected = _hit_point.recover(1);
        assert_eq!(corrected.value, 999)
    }

    #[test]
    fn member_damage() {
        let mut member = Member::new(HitPoint { value: 999 });
        member.damage(1);

        assert_eq!(member.state.to_string(), "[State::Normal]");
    }

    #[test]
    fn member_damage_zero() {
        let mut member = Member::new(HitPoint { value: 999 });
        member.damage(999);

        assert_eq!(member.state.to_string(), "[State::Dead]");
    }
}
