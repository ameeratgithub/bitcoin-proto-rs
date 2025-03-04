use std::fmt;
use std::ops::Add;

use crate::ecc::field_element::FieldElement;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Point {
    a: FieldElement,
    b: FieldElement,
    x: Option<FieldElement>,
    y: Option<FieldElement>,
}

impl Point {
    pub fn new(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
        a: FieldElement,
        b: FieldElement,
    ) -> Result<Self, String> {
        if x.is_some() && y.is_some() {
            let y2 = y.unwrap().field_power(2)?;
            let x3 = x.unwrap().field_power(3)?;
            let a = (a * x.unwrap())?;
            let rhs = (x3 + (a + b)?)?;

            if y2 != rhs {
                return Err(format!(
                    "({},{}) is not on the curve",
                    x.unwrap(),
                    y.unwrap()
                ));
            }
        }

        Ok(Self { a, b, x, y })
    }
}

impl Add for Point {
    type Output = Result<Point, String>;

    fn add(self, other: Self) -> Self::Output {
        if self.a != other.a || self.b != other.b {
            return Err(format!(
                "Points {}, {} are not on the same curve",
                self, other
            ));
        }

        if self.x.is_none() {
            Ok(other)
        } else if other.x.is_none() {
            Ok(self)
        } else if self.x == other.x && self.y != other.y {
            Point::new(None, None, self.a, self.b)
        } else if self.x != other.x {
            let x1 = self.x.unwrap();
            let x2 = other.x.unwrap();

            let y1 = self.y.unwrap();
            let y2 = other.y.unwrap();

            let s = ((y2 - y1)? / (x2 - x1)?)?;

            let x3 = ((s.field_power(2)? - x1)? - x2)?;
            let y3 = ((s * (x1 - x3)?)? - y1)?;

            Point::new(Some(x3), Some(y3), self.a, self.b)
        } else if self == other && self.y.unwrap().num == 0 {
            // Points are equal and y coordinate is zero.
            // We can't calculate slope here
            Point::new(None, None, self.a, self.b)
        } else if self == other {
            let x1 = self.x.unwrap();
            let y1 = self.y.unwrap();

            let p1 = 3 * (x1.field_power(2)?).num + self.a.num;
            let p2 = 2 * y1.num;
            let s = FieldElement::new(p1 / p2, x1.prime)?;

            let x3 = (s.field_power(2)? - (FieldElement::new(2, x1.prime)? * x1)?)?;
            let y3 = ((s * (x1 - x3)?)? - y1)?;

            Point::new(Some(x3), Some(y3), self.a, self.b)
        } else {
            Err("".to_string())
        }
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.x.unwrap(),
            self.y.unwrap(),
            self.a,
            self.b
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::ecc::field_element::FieldElement;

    use super::Point;

    #[test]
    fn setup() {
        let prime = 223;
        let a = FieldElement::new(5, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();

        let x1 = Some(FieldElement::new(-1, prime).unwrap());
        let y1 = Some(FieldElement::new(-1, prime).unwrap());

        let x2 = Some(FieldElement::new(-1, prime).unwrap());
        let y2 = Some(FieldElement::new(-2, prime).unwrap());

        let p1 = Point::new(x1, y1, a, b);
        let p2 = Point::new(x2, y2, a, b);

        assert!(p1.is_ok());
        assert!(p2.is_err());

        let x1 = Some(FieldElement::new(2, prime).unwrap());
        let y1 = Some(FieldElement::new(4, prime).unwrap());

        let x2 = Some(FieldElement::new(-1, prime).unwrap());
        let y2 = Some(FieldElement::new(-1, prime).unwrap());

        let x3 = Some(FieldElement::new(18, prime).unwrap());
        let y3 = Some(FieldElement::new(77, prime).unwrap());

        let x4 = Some(FieldElement::new(5, prime).unwrap());
        let y4 = Some(FieldElement::new(7, prime).unwrap());

        let p1 = Point::new(x1, y1, a, b);
        let p2 = Point::new(x2, y2, a, b);
        let p3 = Point::new(x3, y3, a, b);
        let p4 = Point::new(x4, y4, a, b);

        assert!(p1.is_err());
        assert!(p2.is_ok());
        assert!(p3.is_ok());
        assert!(p4.is_err());
    }

    #[test]
    fn addition() {
        let prime = 223;
        let a = FieldElement::new(5, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();

        // p1.x != p2.x

        let x1 = Some(FieldElement::new(2, prime).unwrap());
        let y1 = Some(FieldElement::new(5, prime).unwrap());

        let x2 = Some(FieldElement::new(-1, prime).unwrap());
        let y2 = Some(FieldElement::new(-1, prime).unwrap());

        let p1 = Point::new(x1, y1, a, b).unwrap();
        let p2 = Point::new(x2, y2, a, b).unwrap();
        assert_eq!(format!("{}", (p1 + p2).unwrap()), "(3, 216, 5, 7)");

        // p1 == p2
        let x1 = Some(FieldElement::new(-1, prime).unwrap());
        let y1 = Some(FieldElement::new(-1, prime).unwrap());

        let x2 = Some(FieldElement::new(-1, prime).unwrap());
        let y2 = Some(FieldElement::new(-1, prime).unwrap());

        let p1 = Point::new(x1, y1, a, b).unwrap();
        let p2 = Point::new(x2, y2, a, b).unwrap();
        assert_eq!(format!("{}", (p1 + p2).unwrap()), "(18, 77, 5, 7)");
    }

    #[test]
    fn test_on_curve() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();

        let valid_points = vec![(192, 105), (17, 56), (1, 193)];
        let invalid_points = vec![(200, 119), (42, 99)];

        for (x, y) in valid_points {
            let x = FieldElement::new(x, prime).unwrap();
            let y = FieldElement::new(y, prime).unwrap();

            let p = Point::new(Some(x), Some(y), a, b);

            assert!(p.is_ok());
        }

        for (x, y) in invalid_points {
            let x = FieldElement::new(x, prime).unwrap();
            let y = FieldElement::new(y, prime).unwrap();

            let p = Point::new(Some(x), Some(y), a, b);
            assert!(p.is_err());
        }
    }

    #[test]
    fn point_addition_over_finite_field() {
        let prime = 223;

        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();

        let x1 = FieldElement::new(192, prime).unwrap();
        let y1 = FieldElement::new(105, prime).unwrap();
        let x2 = FieldElement::new(17, prime).unwrap();
        let y2 = FieldElement::new(56, prime).unwrap();

        let p1 = Point::new(Some(x1), Some(y1), a, b).unwrap();
        let p2 = Point::new(Some(x2), Some(y2), a, b).unwrap();

        assert_eq!(format!("{}", (p1 + p2).unwrap()), "(170, 142, 0, 7)");

        let x1 = FieldElement::new(170, prime).unwrap();
        let y1 = FieldElement::new(142, prime).unwrap();
        let x2 = FieldElement::new(60, prime).unwrap();
        let y2 = FieldElement::new(139, prime).unwrap();

        let p1 = Point::new(Some(x1), Some(y1), a, b).unwrap();
        let p2 = Point::new(Some(x2), Some(y2), a, b).unwrap();

        assert_eq!(format!("{}", (p1 + p2).unwrap()), "(220, 181, 0, 7)");

        let x1 = FieldElement::new(47, prime).unwrap();
        let y1 = FieldElement::new(71, prime).unwrap();
        let x2 = FieldElement::new(17, prime).unwrap();
        let y2 = FieldElement::new(56, prime).unwrap();

        let p1 = Point::new(Some(x1), Some(y1), a, b).unwrap();
        let p2 = Point::new(Some(x2), Some(y2), a, b).unwrap();

        assert_eq!(format!("{}", (p1 + p2).unwrap()), "(215, 68, 0, 7)");

        let x1 = FieldElement::new(143, prime).unwrap();
        let y1 = FieldElement::new(98, prime).unwrap();
        let x2 = FieldElement::new(76, prime).unwrap();
        let y2 = FieldElement::new(66, prime).unwrap();

        let p1 = Point::new(Some(x1), Some(y1), a, b).unwrap();
        let p2 = Point::new(Some(x2), Some(y2), a, b).unwrap();

        assert_eq!(format!("{}", (p1 + p2).unwrap()), "(47, 71, 0, 7)");
    }
}
