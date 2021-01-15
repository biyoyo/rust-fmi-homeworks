use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bigint {
    pub digits: Vec<u8>,
}

impl FromStr for Bigint {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = Vec::with_capacity(s.len());

        for c in s.chars() {
            if let Some(digit) = c.to_digit(10) {
                digits.push(digit as u8);
            } else {
                return Err("Invalid input!");
            }
        }

        Ok(Bigint { digits })
    }
}

use std::fmt;

impl fmt::Display for Bigint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.digits.is_empty() {
            write!(f, "{}", 0)?;
        } else {
            for n in &self.digits {
                write!(f, "{}", n)?;
            }
        }
        Ok(())
    }
}

pub struct Delimited<'a> {
    bigint: &'a Bigint,
}

impl Bigint {
    pub fn delimited(&self) -> Delimited {
        Delimited { bigint: self }
    }
}

impl<'a> fmt::Display for Delimited<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.bigint.digits.is_empty() {
            write!(f, "{}", 0)?;
        } else {
            let len = self.bigint.digits.len();
            for n in 0..len {
                write!(f, "{}", self.bigint.digits[n])?;
                if (len - 1 - n) % 3 == 0 && n != len - 1 {
                    write!(f, "{}", ',')?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Bigint;
    use std::str::FromStr;
    #[test]
    fn basic_display() {
        let bi = Bigint::from_str("1000").unwrap();
        assert_eq!(format!("{}", bi), "1000");
        let bi = Bigint::from_str("0").unwrap();
        assert_eq!(format!("{}", bi), "0");
        let bi = Bigint::from_str("15688454648").unwrap();
        assert_eq!(format!("{}", bi), "15688454648");
        let bi = Bigint::from_str("15688454648").unwrap();
        assert_eq!(format!("{}", bi.delimited()), "15,688,454,648");
        let bi = Bigint::from_str("0").unwrap();
        assert_eq!(format!("{}", bi.delimited()), "0");
        let bi = Bigint::from_str("10").unwrap();
        assert_eq!(format!("{}", bi.delimited()), "10");
        let bi = Bigint::from_str("1000").unwrap();
        assert_eq!(format!("{}", bi.delimited()), "1,000");
        let bi = Bigint::from_str("10000").unwrap();
        assert_eq!(format!("{}", bi.delimited()), "10,000");
        let bi = Bigint::from_str("100000000000").unwrap();
        assert_eq!(format!("{}", bi.delimited()), "100,000,000,000");
        let bi = Bigint::from_str("10000000").unwrap();
        assert_eq!(format!("{}", bi.delimited()), "10,000,000");
        let bi = Bigint::from_str("1").unwrap();
        assert_eq!(format!("{}", bi.delimited()), "1");
    }
}
