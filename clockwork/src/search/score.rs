use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::SubAssign;

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct Score(pub i32);

impl Score {
    pub const INVALID: Self = Self(-50000);

    pub const INFINITY: Self = Self(50000);

    pub const CHECKMATE: Self = Self(48000);
    pub const CHECKMATE_LOWER_BOUND: Self = Self(47500);

    pub const DRAW: Self = Self(0);

    pub fn is_mating(self) -> bool {
        self > Self::CHECKMATE_LOWER_BOUND
    }

    pub fn is_getting_mated(self) -> bool {
        self < -Self::CHECKMATE_LOWER_BOUND
    }

    pub fn checkmate_in(self) -> Option<i32> {
        if self.is_mating() {
            // (CHECKMATE.0 - self.0) gives ply. (ply + 1) / 2 gives moves.
            return Some((Score::CHECKMATE.0 - self.0 + 1) / 2);
        }
        if self.is_getting_mated() {
            // This maps, e.g., -48000 (mated in 0) to 0.
            // And -47998 (mated in 2 ply) to -1 (mated in 1 move).
            return Some((-Score::CHECKMATE.0 - self.0) / 2);
        }
        None
    }
}

impl Neg for Score {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Mul<i32> for Score {
    type Output = Self;

    fn mul(self, other: i32) -> Self::Output {
        Self(self.0 * other)
    }
}

impl Add<i32> for Score {
    type Output = Self;

    fn add(self, other: i32) -> Self::Output {
        Self(self.0 + other)
    }
}

impl AddAssign<i32> for Score {
    fn add_assign(&mut self, rhs: i32){
        self.0 += rhs;
    }
}

impl SubAssign<i32> for Score {
    fn sub_assign(&mut self, rhs: i32){
        self.0 -= rhs;
    }
}