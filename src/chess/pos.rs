use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub const fn new(x: isize, y: isize) -> Self {
        Pos { x, y }
    }

    pub fn x(&self) -> isize {
        self.x
    }

    pub fn y(&self) -> isize {
        self.y
    }

    pub fn distance(&self, other: Pos) -> f32 {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();

        ((dx.pow(2) as f32) + (dy.pow(2) as f32)).sqrt()
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", (self.y as u8 + 65) as char, 8 - self.x)
    }
}

impl Sub for Pos {
    type Output = Shift;

    fn sub(self, rhs: Self) -> Self::Output {
        Shift {
            dx: self.x - rhs.x,
            dy: self.y - rhs.y,
        }
    }
}

impl Add<Shift> for Pos {
    type Output = Pos;

    fn add(self, rhs: Shift) -> Self::Output {
        Pos {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
        }
    }
}

impl AddAssign<Shift> for Pos {
    fn add_assign(&mut self, rhs: Shift) {
        self.x = self.x + rhs.dx;
        self.y = self.y + rhs.dy;
    }
}

impl Sub<Shift> for Pos {
    type Output = Pos;

    fn sub(self, rhs: Shift) -> Self::Output {
        Pos {
            x: self.x - rhs.dx,
            y: self.y - rhs.dy,
        }
    }
}

impl SubAssign<Shift> for Pos {
    fn sub_assign(&mut self, rhs: Shift) {
        self.x = self.x - rhs.dx;
        self.y = self.y - rhs.dy;
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Shift {
    dx: isize,
    dy: isize,
}

impl Shift {
    pub const UP: Shift = Shift { dx: 0, dy: -1 };
    pub const UP_RIGHT: Shift = Shift { dx: 1, dy: -1 };
    pub const RIGHT: Shift = Shift { dx: 1, dy: 0 };
    pub const DOWN_RIGHT: Shift = Shift { dx: 1, dy: 1 };
    pub const DOWN: Shift = Shift { dx: 0, dy: 1 };
    pub const DOWN_LEFT: Shift = Shift { dx: -1, dy: 1 };
    pub const LEFT: Shift = Shift { dx: -1, dy: 0 };
    pub const UP_LEFT: Shift = Shift { dx: -1, dy: -1 };

    pub const CARDINAL_DIRS: [Shift; 4] = [Self::UP, Self::RIGHT, Self::DOWN, Self::LEFT];

    pub const DIAGONAL_DIRS: [Shift; 4] = [
        Self::UP_RIGHT,
        Self::DOWN_RIGHT,
        Self::DOWN_LEFT,
        Self::UP_LEFT,
    ];

    pub const DIRS: [Shift; 8] = [
        Self::UP,
        Self::UP_RIGHT,
        Self::RIGHT,
        Self::DOWN_RIGHT,
        Self::DOWN,
        Self::DOWN_LEFT,
        Self::LEFT,
        Self::UP_LEFT,
    ];

    pub const JUMPS: [Shift; 8] = [
        Shift { dx: 1, dy: -2 },
        Shift { dx: 2, dy: -1 },
        Shift { dx: 2, dy: 1 },
        Shift { dx: 1, dy: 2 },
        Shift { dx: -1, dy: 2 },
        Shift { dx: -2, dy: 1 },
        Shift { dx: -2, dy: -1 },
        Shift { dx: -1, dy: -2 },
    ];
}

impl Neg for Shift {
    type Output = Shift;

    fn neg(self) -> Self::Output {
        Shift {
            dx: -self.dx,
            dy: -self.dy,
        }
    }
}

impl Add for Shift {
    type Output = Shift;

    fn add(self, rhs: Self) -> Self::Output {
        Shift {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
        }
    }
}

impl AddAssign for Shift {
    fn add_assign(&mut self, rhs: Self) {
        self.dx += rhs.dx;
        self.dy += rhs.dy;
    }
}

impl Sub for Shift {
    type Output = Shift;

    fn sub(self, rhs: Self) -> Self::Output {
        Shift {
            dx: self.dx - rhs.dx,
            dy: self.dy - rhs.dy,
        }
    }
}

impl SubAssign for Shift {
    fn sub_assign(&mut self, rhs: Self) {
        self.dx -= rhs.dx;
        self.dy -= rhs.dy;
    }
}

impl Mul<isize> for Shift {
    type Output = Shift;

    fn mul(self, rhs: isize) -> Self::Output {
        Shift {
            dx: self.dx * rhs,
            dy: self.dy * rhs,
        }
    }
}

impl MulAssign<isize> for Shift {
    fn mul_assign(&mut self, rhs: isize) {
        self.dx *= rhs;
        self.dy *= rhs;
    }
}

impl Mul<Shift> for isize {
    type Output = Shift;

    fn mul(self, rhs: Shift) -> Self::Output {
        Shift {
            dx: rhs.dx * self,
            dy: rhs.dy * self,
        }
    }
}
