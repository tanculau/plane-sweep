// Based on Geometriekalküle from  Jürgen Richter-Gebert, Thorsten Orendt https://doi.org/10.1007/978-3-642-02530-3

use core::{fmt::Display, ops::Neg};

use tracing::{debug, instrument};

use crate::math::{A, CrossProduct, DotProduct, calculate_multiple, homogeneous::HomogeneousCoord};

#[derive(Debug, Clone, Copy)]
pub struct Line<T: A> {
    pub a: T,
    pub b: T,
    pub c: T,
}

impl<T: A> PartialOrd for Line<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.a.partial_cmp(&other.a) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.b.partial_cmp(&other.b) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.c.partial_cmp(&other.c)
    }
}

impl<T: A> Neg for Line<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            a: -self.a,
            b: -self.b,
            c: -self.c,
        }
    }
}

impl<T: A> PartialEq for Line<T> {
    fn eq(&self, other: &Self) -> bool {
        let a = calculate_multiple(&self.a, &other.a);
        let b = calculate_multiple(&self.b, &other.b);
        let c = calculate_multiple(&self.c, &other.c);
        a == b && b == c
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Slope<T> {
    ThirdQuadrant(T),
    Vertical,
    FourthQuadrant(T),
    Horizontal,
    Infinity,
}
impl<T: Display> Display for Slope<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ThirdQuadrant(float) => {
                write!(f, "3. Quadrant({float:.2})")
            }
            Self::Vertical => write!(f, "Vertical"),
            Self::FourthQuadrant(float) => {
                write!(f, "4. Quadrant({float:.2})")
            }
            Self::Horizontal => write!(f, "Horizontal"),
            Self::Infinity => write!(f, "Infinity"),
        }
    }
}

impl<T: A> Line<T> {
    #[must_use]
    pub fn x_axis() -> Self {
        Self {
            a: 0.into(),
            b: 1.into(),
            c: 0.into(),
        }
    }
    #[must_use]
    pub fn y_axis() -> Self {
        Self {
            a: 1.into(),
            b: 0.into(),
            c: 0.into(),
        }
    }
    pub fn new(a: impl Into<T>, b: impl Into<T>, c: impl Into<T>) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
        }
    }

    pub fn horizontal(y: impl Into<T>) -> Self {
        Self::new(0, -1, y.into())
    }

    pub fn vertical(x: impl Into<T>) -> Self {
        Self::new(-1, 0, x.into())
    }

    #[must_use]
    pub const fn tuple(&self) -> (&T, &T, &T) {
        (&self.a, &self.b, &self.c)
    }
    #[must_use]
    pub fn tuple_owned(&self) -> (T, T, T) {
        (self.a.clone(), self.b.clone(), self.c.clone())
    }

    #[must_use]
    pub const fn array(&self) -> [&T; 3] {
        [&self.a, &self.b, &self.c]
    }
}

impl<T: A> Line<T> {
    #[must_use]
    pub fn intersection(self, other: Self) -> HomogeneousCoord<T> {
        self.cross_product(other)
    }
}

impl<T: A> Line<T> {
    #[instrument(name = "Line::contains_coord", skip(self, coord))]
    pub fn contains_coord(self, coord: impl Into<HomogeneousCoord<T>>) -> bool {
        let coord = coord.into();
        let res = self.tuple_owned().dot_product(coord.tuple());

        res == 0.into()
    }
}

impl<T: A> Line<T> {
    #[must_use]
    pub fn slope(self) -> Slope<T> {
        match ((self.a.is_zero()), (self.b.is_zero())) {
            (true, true) => Slope::Infinity,
            (true, false) => Slope::Horizontal,
            (false, true) => Slope::Vertical,
            (false, false) => {
                let slope = -self.a / &self.b;

                if slope > 0.into() {
                    Slope::ThirdQuadrant(slope)
                } else {
                    Slope::FourthQuadrant(slope)
                }
            }
        }
    }
}

impl<T: A> Line<T> {
    #[must_use]
    pub fn angle(self) -> T {
        let Self { a: a1, b: b1, .. } = self;
        -a1 / &b1
    }
}

impl<T: A, TA: Into<T>, TB: Into<T>, TC: Into<T>> From<(TA, TB, TC)> for Line<T> {
    fn from((a, b, c): (TA, TB, TC)) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
        }
    }
}

impl<T: A> CrossProduct for Line<T> {
    type Output = HomogeneousCoord<T>;

    fn cross_product(self, rhs: Self) -> Self::Output {
        let result = self.tuple_owned().cross_product(rhs.tuple()).into();
        debug!("Cross product of lines {self:?} and {rhs:?} is {result:?}");
        result
    }
}

impl<T: A> DotProduct for Line<T> {
    type Output = T;

    fn dot_product(self, rhs: Self) -> Self::Output {
        self.tuple_owned().dot_product(rhs.tuple())
    }
}
