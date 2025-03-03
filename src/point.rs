use std::fmt;
use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Point {
    a: i32,
    b: i32,
    x: Option<i32>,
    y: Option<i32>,
}

impl Point {
    pub fn new(x: Option<i32>, y: Option<i32>, a: i32, b: i32) -> Result<Self, String> {
        if x.is_some() && y.is_some() {
            if y.unwrap().pow(2) != x.unwrap().pow(3) + a * x.unwrap() + b {
                return Err(format!(
                    "({},{}) is not on the curve",
                    x.unwrap(),
                    y.unwrap()
                ));
            }
        }

        Ok(Self { a, b, x, y })
    }

    pub fn add(&self, other: Point) -> Result<Point, String> {
        if self.a != other.a || self.b != other.b {
            return Err(format!(
                "Points {}, {} are not on the same curve",
                self, other
            ));
        }

        if self.x.is_none() {
            return Ok(other);
        } else if other.x.is_none() {
            return Ok(*self);
        } else if self.x == other.x && self.y != other.y {
            return Ok(Point::new(None, None, self.a, self.b).unwrap());
        } else if self.x != other.x {
            let x1 = self.x.unwrap();
            let x2 = other.x.unwrap();

            let y1 = self.y.unwrap();
            let y2 = other.y.unwrap();

            let s = (y2 - y1) / (x2 - x1);

            let x3 = s.pow(2) - x1 - x2;
            let y3 = s * (x1 - x3) - y1;

            return Ok(Point::new(Some(x3), Some(y3), self.a, self.b)?);
        } else if *self == other {
            let x1 = self.x.unwrap();
            let y1 = self.y.unwrap();

            let s = (3 * x1.pow(2) + self.a) / 2 * y1;

            let x3 = s.pow(2) - 2 * x1;
            let y3 = s * (x1 - x3) - y1;

            return Ok(Point::new(Some(x3), Some(y3), self.a, self.b)?);
        } else {
            return Err("".to_string());
        }
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
            return Ok(other);
        } else if other.x.is_none() {
            return Ok(self);
        } else if self.x == other.x && self.y != other.y {
            return Ok(Point::new(None, None, self.a, self.b).unwrap());
        } else if self.x != other.x {
            let x1 = self.x.unwrap();
            let x2 = other.x.unwrap();

            let y1 = self.y.unwrap();
            let y2 = other.y.unwrap();

            let s = (y2 - y1) / (x2 - x1);

            let x3 = s.pow(2) - x1 - x2;
            let y3 = s * (x1 - x3) - y1;

            return Ok(Point::new(Some(x3), Some(y3), self.a, self.b)?);
        } else if self == other && self.y.unwrap() == 0 * self.x.unwrap() {
            // Points are equal and y coordinate is zero.
            // We can't calculate slope here
            return Ok(Point::new(None, None, self.a, self.b)?);
        } else if self == other {
            let x1 = self.x.unwrap();
            let y1 = self.y.unwrap();

            let s = (3 * x1.pow(2) + self.a) / 2 * y1;

            let x3 = s.pow(2) - 2 * x1;
            let y3 = s * (x1 - x3) - y1;

            return Ok(Point::new(Some(x3), Some(y3), self.a, self.b)?);
        } else {
            return Err("".to_string());
        }
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x.unwrap(), self.y.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::field_element::FieldElement;

    use super::Point;

    #[test]
    fn setup() {
        let p1 = Point::new(Some(-1), Some(-1), 5, 7);
        let p2 = Point::new(Some(-1), Some(-2), 5, 7);

        assert!(p1.is_ok());
        assert!(p2.is_err());

        let p1 = Point::new(Some(2), Some(4), 5, 7);
        let p2 = Point::new(Some(-1), Some(-1), 5, 7);
        let p3 = Point::new(Some(18), Some(77), 5, 7);
        let p4 = Point::new(Some(5), Some(7), 5, 7);

        assert!(p1.is_err());
        assert!(p2.is_ok());
        assert!(p3.is_ok());
        assert!(p4.is_err());
    }

    #[test]
    fn addition() {
        // p1.x != p2.x
        let p1 = Point::new(Some(2), Some(5), 5, 7).unwrap();
        let p2 = Point::new(Some(-1), Some(-1), 5, 7).unwrap();
        assert_eq!(format!("{}", (p1 + p2).unwrap()), "(3, -7)");

        // p1 == p2
        let p1 = Point::new(Some(-1), Some(-1), 5, 7).unwrap();
        let p2 = Point::new(Some(-1), Some(-1), 5, 7).unwrap();
        assert_eq!(format!("{}", (p1 + p2).unwrap()), "(18, 77)");
    }

}
