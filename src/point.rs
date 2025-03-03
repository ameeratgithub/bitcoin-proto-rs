#[derive(Debug, PartialEq, Eq)]
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
}

#[cfg(test)]
mod tests {
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
}
