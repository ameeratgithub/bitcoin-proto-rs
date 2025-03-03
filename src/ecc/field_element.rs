use std::fmt;

use std::ops::{Add, Div, Mul, Sub};

// use crate::ecc::point::Point;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FieldElement {
    pub num: i32,
    pub prime: i32,
}

impl FieldElement {
    pub fn new(num: i32, prime: i32) -> Result<Self, String> {
        let num = num % prime;

        if num >= prime {
            return Err(format!("Num {} not in field range 0 to {}", num, prime - 1));
        }

        Ok(Self { num, prime })
    }

    pub fn field_power(&self, exponent: i32) -> Result<Self, String> {
        let n = exponent.rem_euclid(self.prime - 1);
        let num = mod_exp(self.num, n, self.prime);
        Self::new(num, self.prime)
    }
}

impl Add for FieldElement {
    type Output = Result<Self, String>;

    fn add(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            return Err("Can't add two numbers from different Fields".to_string());
        }

        let mut num = (self.num + other.num) % self.prime;

        if num < 0 {
            num += self.prime;
        }

        FieldElement::new(num, self.prime)
    }
}

impl Sub for FieldElement {
    type Output = Result<Self, String>;

    fn sub(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            return Err("Can't add two numbers from different Fields".to_string());
        }

        let mut num = (self.num - other.num) % self.prime;

        if num < 0 {
            num += self.prime;
        }

        FieldElement::new(num, self.prime)
    }
}

impl Div for FieldElement {
    type Output = Result<Self, String>;

    fn div(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            return Err("Can't divide two numbers from different Fields".to_string());
        }

        let mut num = (self.num * (mod_exp(other.num, self.prime - 2, self.prime))) % self.prime;

        if num < 0 {
            num += self.prime;
        }

        FieldElement::new(num, self.prime)
    }
}

impl Mul for FieldElement {
    type Output = Result<Self, String>;

    fn mul(self, other: Self) -> Self::Output {
        if other.num >= self.prime {
            return Err("All numbers should belong to same set".to_string());
        }

        let mut res = self.num * other.num;

        res %= self.prime;

        if res < 0 {
            res += self.prime;
        }

        FieldElement::new(res, self.prime)
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}

fn mod_exp(mut base: i32, mut exp: i32, modulus: i32) -> i32 {
    let mut result = 1;
    base %= modulus; // Reduce base mod p

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }

        base = (base * base) % modulus;

        exp /= 2;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::FieldElement;
    use crate::ecc::field_element::mod_exp;

    #[test]
    fn basic() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(6, 13);

        assert_eq!(a, a);
        assert_ne!(a, b);
    }

    #[test]
    fn field_addition() {
        let a = FieldElement::new(9, 13).unwrap();
        let b = FieldElement::new(10, 13).unwrap();

        let c = a + b;

        assert!(c.is_ok());
        assert_eq!(c.unwrap().num, 6);

        let a = FieldElement::new(-27, 13).unwrap();
        let b = FieldElement::new(-42, 13).unwrap();

        let c = a + b;

        assert!(c.is_ok());
        assert_eq!(c.unwrap().num, 9);
    }

    #[test]
    fn field_subtraction() {
        let a = FieldElement::new(52, 57).unwrap();
        let b = FieldElement::new(30, 57).unwrap();

        let c = a - b;

        assert!(c.is_ok());
        assert_eq!(c.clone().unwrap().num, 22);

        let d = FieldElement::new(38, 57).unwrap();

        let e = c.unwrap() - d;

        assert!(e.is_ok());
        assert_eq!(e.unwrap().num, 41);
    }

    #[test]
    fn multiplication() {
        let a = FieldElement::new(95, 97).unwrap();
        let b = FieldElement::new(45, 97).unwrap();
        let c = FieldElement::new(31, 97).unwrap();
        let res = (a * b).unwrap() * c;

        assert!(res.is_ok());
        assert_eq!(res.unwrap().num, 23);

        let a = FieldElement::new(17, 97).unwrap();
        let b = FieldElement::new(13, 97).unwrap();
        let c = FieldElement::new(19, 97).unwrap();
        let d = FieldElement::new(44, 97).unwrap();
        let res = ((a * b).unwrap() * c).unwrap() * d;

        assert!(res.is_ok());
        assert_eq!(res.unwrap().num, 68);

        let a = FieldElement::new(mod_exp(12, 7, 97), 97).unwrap();
        let b = FieldElement::new(mod_exp(77, 49, 97), 97).unwrap();
        let res = a * b;

        assert!(res.is_ok());
        assert_eq!(res.unwrap().num, 63);
    }

    #[test]
    fn division() {
        let a = FieldElement::new(3, 31).unwrap();
        let b = FieldElement::new(24, 31).unwrap();

        assert_eq!((a / b).unwrap().num, 4);

        let a = FieldElement::new(1, 31).unwrap();
        let b = FieldElement::new(mod_exp(17, 3, 31), 31).unwrap();

        assert_eq!((a / b).unwrap().num, 29);

        let a = FieldElement::new(11, 31).unwrap();
        let b = FieldElement::new(mod_exp(4, 4, 31), 31).unwrap();

        assert_eq!((a / b).unwrap().num, 13);
    }
}
