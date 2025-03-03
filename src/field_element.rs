use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct FieldElement {
    num: i128,
    prime: i128,
}

impl FieldElement {
    fn new(num: i128, prime: i128) -> Result<Self, String> {
        let mut num = num % prime;

        if num < 0 {
            num += prime;
        }

        if num >= prime {
            return Err(format!("Num {} not in field range 0 to {}", num, prime - 1));
        }

        Ok(Self { num, prime })
    }

    pub fn field_addition(&self, other: &FieldElement) -> Result<Self, String> {
        if self.prime != other.prime {
            return Err("Can't add two numbers from different Fields".to_string());
        }

        let mut num = (self.num + other.num) % self.prime;

        if num < 0 {
            num += self.prime;
        }

        Ok(Self {
            num,
            prime: self.prime,
        })
    }

    pub fn field_subtraction(&self, other: &FieldElement) -> Result<Self, String> {
        if self.prime != other.prime {
            return Err("Can't add two numbers from different Fields".to_string());
        }

        let mut num = (self.num - other.num) % self.prime;

        if num < 0 {
            num += self.prime;
        }

        Ok(Self {
            num,
            prime: self.prime,
        })
    }

    pub fn field_division(&self, other: &FieldElement) -> Result<Self, String> {
        if self.prime != other.prime {
            return Err("Can't divide two numbers from different Fields".to_string());
        }

        let mut num = (self.num * (mod_exp(other.num, self.prime - 2, self.prime))) % self.prime;

        if num < 0 {
            num += self.prime;
        }

        Ok(Self {
            num,
            prime: self.prime,
        })
    }

    pub fn field_multiplication(&self, numbers: &Vec<i128>) -> Result<i128, String> {
        let mut res = self.num;

        for n in numbers {
            if *n >= self.prime {
                return Err("All numbers should belong to same set".to_string());
            }

            res *= n;
        }

        res = res % self.prime;

        if res < 0 {
            res += self.prime;
        }

        Ok(res)
    }

    pub fn field_power(&self, exponent: i128) -> Result<Self, String> {
        let n = exponent.rem_euclid(self.prime - 1);
        let num = mod_exp(self.num, n, self.prime);
        Self::new(num, self.prime)
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.num, self.prime)
    }
}

fn mod_exp(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
    let mut result = 1;
    base = base % modulus; // Reduce base mod p

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
    use crate::field_element::mod_exp;

    use super::FieldElement;

    #[test]
    fn basic() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(6, 13);

        assert_eq!(a, a);
        assert_ne!(a, b);
    }

    #[test]
    fn field_addition() {
        let a = FieldElement::new(9, 13);
        let b = FieldElement::new(10, 13);

        let c = a.unwrap().field_addition(&b.unwrap());

        assert!(c.is_ok());
        assert_eq!(c.unwrap().num, 6);

        let a = FieldElement::new(-27, 13);
        let b = FieldElement::new(-42, 13);

        let c = a.unwrap().field_addition(&b.unwrap());

        assert!(c.is_ok());
        assert_eq!(c.unwrap().num, 9);
    }

    #[test]
    fn field_subtraction() {
        let a = FieldElement::new(52, 57);
        let b = FieldElement::new(30, 57);

        let c = a.unwrap().field_subtraction(&b.unwrap());

        assert!(c.is_ok());
        assert_eq!(c.clone().unwrap().num, 22);

        let d = FieldElement::new(38, 57);

        let e = c.clone().unwrap().field_subtraction(&d.unwrap());

        assert!(e.is_ok());
        assert_eq!(e.unwrap().num, 41);
    }

    #[test]
    fn multiplication() {
        let a = FieldElement::new(95, 97);
        let res = a.clone().unwrap().field_multiplication(&vec![45, 31]);

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 23);

        let b = FieldElement::new(17, 97);
        let res = b.clone().unwrap().field_multiplication(&vec![13, 19, 44]);

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 68);

        let c = FieldElement::new(mod_exp(12, 7, 97), 97);
        let res = c
            .clone()
            .unwrap()
            .field_multiplication(&vec![mod_exp(77, 49, 97)]);

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 63);
    }

    #[test]
    fn division() {
        let a = FieldElement::new(3, 31).unwrap();
        let b = FieldElement::new(24, 31).unwrap();

        assert_eq!(a.field_division(&b).unwrap().num, 4);

        let a = FieldElement::new(1, 31).unwrap();
        let b = FieldElement::new(mod_exp(17, 3, 31), 31).unwrap();

        assert_eq!(a.field_division(&b).unwrap().num, 29);

        let a = FieldElement::new(11, 31).unwrap();
        let b = FieldElement::new(mod_exp(4, 4, 31), 31).unwrap();

        assert_eq!(a.field_division(&b).unwrap().num, 13);

        
        
    }
}
