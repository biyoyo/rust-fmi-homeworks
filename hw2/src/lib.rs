#[derive(Debug, PartialEq, Eq)]
pub struct Bigint {
    sign: i8,
    digits: Vec<u8>,
}

impl Bigint {
    pub fn new() -> Self {
        Bigint {
            sign: 1,
            digits: vec![0],
        }
    }

    pub fn is_positive(&self) -> bool {
        match self.digits[0] {
            0 => false,
            _ => self.sign == 1,
        }
    }

    pub fn is_negative(&self) -> bool {
        match self.digits[0] {
            0 => false,
            _ => self.sign == -1,
        }
    }
}

use std::str::FromStr;

#[derive(Debug)]
pub struct ParseError;

impl FromStr for Bigint {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Bigint::new());
        }

        let mut input: Vec<u8> = s.bytes().collect();

        let sign = if input[0] == b'+' {
            input.remove(0);
            1
        } else if input[0] == b'-' {
            input.remove(0);
            -1
        } else {
            1
        };

        while input.len() > 1 && input[0] == b'0' {
            input.remove(0);
        }

        if input.len() == 1 && input[0] == b'0' {
            return Ok(Bigint::new());
        }

        if input.is_empty() {
            return Err(ParseError);
        }

        for i in input.iter_mut() {
            if *i < b'0' || *i > b'9' {
                return Err(ParseError);
            }

            *i -= b'0';
        }
        Ok(Bigint {
            sign: sign,
            digits: input,
        })
    }
}

use std::cmp::Ordering;

impl PartialOrd for Bigint {
    fn partial_cmp(&self, other: &Bigint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bigint {
    fn cmp(&self, other: &Bigint) -> Ordering {
        if self.sign != other.sign {
            return match self.sign {
                -1 => Ordering::Less,
                1 => Ordering::Greater,
                _ => unreachable!(),
            };
        }

        let order = if self.digits.len() != other.digits.len() {
            self.digits.len().cmp(&other.digits.len())
        } else {
            self.digits.cmp(&other.digits)
        };

        if self.sign == 1 {
            order
        } else {
            order.reverse()
        }
    }
}

use std::ops::{Add, Sub};

impl Add for Bigint {
    type Output = Bigint;

    fn add(self, other: Self) -> Self {
        let mut result = Vec::<u8>::new();
        let mut sign = self.sign;

        if self.sign == other.sign {
            let mut i = self.digits.len();
            let mut j = other.digits.len();

            let mut rem = false;

            while i != 0 && j != 0 {
                let num = self.digits[i - 1] + other.digits[j - 1] + rem as u8;
                result.insert(0, num % 10);
                rem = num >= 10;
                i -= 1;
                j -= 1;
            }

            while i != 0 {
                i -= 1;
                let num = self.digits[i] + rem as u8;
                result.insert(0, num % 10);
                rem = num >= 10;
            }

            while j != 0 {
                j -= 1;
                let num = other.digits[j] + rem as u8;
                result.insert(0, num % 10);
                rem = num >= 10;
            }
        } else {
            let (bigger, smaller) = if self.digits.len() == other.digits.len() {
                match self.digits.cmp(&other.digits) {
                    Ordering::Greater => (&self, &other),
                    Ordering::Less => (&other, &self),
                    Ordering::Equal => return Bigint::new(),
                }
            } else {
                match self.digits.len() > other.digits.len() {
                    true => (&self, &other),
                    false => (&other, &self),
                }
            };

            let mut i = bigger.digits.len();
            let mut j = smaller.digits.len();

            let mut flag1;
            let mut flag2 = false;

            while i != 0 && j != 0 {
                flag1 = (bigger.digits[i - 1] < smaller.digits[j - 1])
                    || (bigger.digits[i - 1] == smaller.digits[j - 1] && flag2 == true);
                let num =
                    10 * flag1 as u8 + bigger.digits[i - 1] - smaller.digits[j - 1] - flag2 as u8;
                flag2 = flag1;
                result.insert(0, num);
                i -= 1;
                j -= 1;
            }

            while i != 0 {
                i -= 1;
                result.insert(0, bigger.digits[i] - flag2 as u8);
                flag2 = false;
            }

            while j != 0 {
                j -= 1;
                result.insert(0, smaller.digits[j] - flag2 as u8);
                flag2 = false;
            }

            while result[0] == 0 {
                result.remove(0);
            }

            sign = bigger.sign;
        }
        return Bigint {
            sign: sign,
            digits: result,
        };
    }
}

impl Sub for Bigint {
    type Output = Bigint;

    fn sub(self, other: Self) -> Self {
        let new_other = Bigint {
            sign: -other.sign,
            digits: other.digits,
        };
        self.add(new_other)
    }
}

#[cfg(test)]
mod tests {
    use crate::Bigint;
    use std::cmp::Ordering;
    use std::ops::{Add, Sub};
    use std::str::FromStr;

    #[test]
    fn pos_neg() {
        let a = vec![1, 2, 3];
        let b = vec![1, 2, 3];

        let pos = Bigint { sign: 1, digits: a };
        let neg = Bigint {
            sign: -1,
            digits: b,
        };

        assert_eq!(pos.is_negative(), false);
        assert_eq!(neg.is_negative(), true);
        assert_eq!(pos.is_positive(), true);
        assert_eq!(neg.is_positive(), false);

        let c = vec![0];
        let zero = Bigint { sign: 1, digits: c };
        assert_eq!(zero.is_positive(), false);
        assert_eq!(zero.is_negative(), false);
    }

    #[test]
    fn test_from_string() {
        let a = Bigint::from_str("").unwrap();
        assert_eq!(a.digits, vec![0]);
        assert_eq!(a.sign, 1);

        let a = Bigint::from_str("123").unwrap();
        assert_eq!(a.digits, vec![1, 2, 3]);
        assert_eq!(a.sign, 1);

        let a = Bigint::from_str("-123").unwrap();
        assert_eq!(a.digits, vec![1, 2, 3]);
        assert_eq!(a.sign, -1);

        let a = Bigint::from_str("+123").unwrap();
        assert_eq!(a.digits, vec![1, 2, 3]);
        assert_eq!(a.sign, 1);

        let a = Bigint::from_str("-0").unwrap();
        assert_eq!(a, Bigint::new());
        assert_eq!(a.digits, vec![0]);
        assert_eq!(a.sign, 1);

        let a = Bigint::from_str("+0").unwrap();
        assert_eq!(a, Bigint::new());
        assert_eq!(a.digits, vec![0]);
        assert_eq!(a.sign, 1);

        let a = Bigint::from_str("+00000000013").unwrap();
        assert_eq!(a.digits, vec![1, 3]);
        assert_eq!(a.sign, 1);

        let a = Bigint::from_str("+0000").unwrap();
        assert_eq!(a, Bigint::new());
        assert_eq!(a.digits, vec![0]);
        assert_eq!(a.sign, 1);

        let a = Bigint::from_str("-0000").unwrap();
        assert_eq!(a, Bigint::new());
        assert_eq!(a.digits, vec![0]);
        assert_eq!(a.sign, 1);

        let a = Bigint::from_str("0").unwrap();
        assert_eq!(a.digits, vec![0]);
        assert_eq!(a.sign, 1);

        assert!(Bigint::from_str("abc").is_err());
        assert!(Bigint::from_str("1+2").is_err());
        assert!(Bigint::from_str("   12").is_err());
        assert!(Bigint::from_str("+1   ").is_err());
        assert!(Bigint::from_str("-").is_err());
    }

    #[test]
    fn test_order() {
        let a = Bigint::from_str("-1").unwrap();
        let b = Bigint::from_str("2").unwrap();
        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);

        let a = Bigint::from_str("1").unwrap();
        let b = Bigint::from_str("2").unwrap();
        assert_eq!(a.cmp(&b), Ordering::Less);

        let a = Bigint::from_str("-1").unwrap();
        let b = Bigint::from_str("-2").unwrap();
        assert_eq!(a.cmp(&b), Ordering::Greater);

        let a = Bigint::from_str("10").unwrap();
        let b = Bigint::from_str("2").unwrap();
        assert_eq!(a.cmp(&b), Ordering::Greater);
        assert_eq!(b.cmp(&a), Ordering::Less);

        let a = Bigint::from_str("-10").unwrap();
        let b = Bigint::from_str("-2").unwrap();
        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);

        let a = Bigint::from_str("-0").unwrap();
        let b = Bigint::from_str("+0").unwrap();
        assert_eq!(a.cmp(&b), Ordering::Equal);

        let a = Bigint::from_str("0").unwrap();
        let b = Bigint::from_str("+0").unwrap();
        assert_eq!(a.cmp(&b), Ordering::Equal);

        let a = Bigint::from_str("0").unwrap();
        let b = Bigint::from_str("50").unwrap();
        assert_eq!(a.cmp(&b), Ordering::Less);

        let a = Bigint::from_str("+5000").unwrap();
        let b = Bigint::from_str("5000").unwrap();
        assert_eq!(a.cmp(&b), Ordering::Equal);
    }

    #[test]
    fn test_add_and_sub() {
        assert_eq!(
            Bigint::from_str("21325")
                .unwrap()
                .add(Bigint::from_str("12").unwrap())
                .digits,
            vec![2, 1, 3, 3, 7]
        );
        assert_eq!(
            Bigint::from_str("123456")
                .unwrap()
                .add(Bigint::from_str("444").unwrap())
                .digits,
            vec![1, 2, 3, 9, 0, 0]
        );
        assert_eq!(
            Bigint::from_str("15")
                .unwrap()
                .add(Bigint::from_str("15").unwrap())
                .digits,
            vec![3, 0]
        );
        assert_eq!(
            Bigint::from_str("+25")
                .unwrap()
                .add(Bigint::from_str("17").unwrap())
                .digits,
            vec![4, 2]
        );
        assert_eq!(
            Bigint::from_str("-25")
                .unwrap()
                .add(Bigint::from_str("-17").unwrap()),
            Bigint {
                sign: -1,
                digits: vec![4, 2]
            }
        );
        assert_eq!(
            Bigint::from_str("21325")
                .unwrap()
                .add(Bigint::from_str("-12").unwrap())
                .digits,
            vec![2, 1, 3, 1, 3]
        );
        assert_eq!(
            Bigint::from_str("25")
                .unwrap()
                .add(Bigint::from_str("-12").unwrap())
                .digits,
            vec![1, 3]
        );
        assert_eq!(
            Bigint::from_str("325")
                .unwrap()
                .add(Bigint::from_str("-98").unwrap())
                .digits,
            vec![2, 2, 7]
        );
        assert_eq!(
            Bigint::from_str("25")
                .unwrap()
                .add(Bigint::from_str("-25").unwrap())
                .digits,
            vec![0]
        );
        assert_eq!(
            Bigint::from_str("100")
                .unwrap()
                .add(Bigint::from_str("-99").unwrap()),
            Bigint {
                sign: 1,
                digits: vec![1]
            }
        );
        assert_eq!(
            Bigint::from_str("-25")
                .unwrap()
                .add(Bigint::from_str("25").unwrap()),
            Bigint {
                sign: 1,
                digits: vec![0]
            }
        );

        assert_eq!(
            Bigint::from_str("17")
                .unwrap()
                .sub(Bigint::from_str("19").unwrap()),
            Bigint {
                sign: -1,
                digits: vec![2]
            }
        );
        assert_eq!(
            Bigint::from_str("325")
                .unwrap()
                .sub(Bigint::from_str("98").unwrap())
                .digits,
            vec![2, 2, 7]
        );
        assert_eq!(
            Bigint::from_str("-25")
                .unwrap()
                .sub(Bigint::from_str("-25").unwrap())
                .digits,
            vec![0]
        );
        assert_eq!(
            Bigint::from_str("325")
                .unwrap()
                .sub(Bigint::from_str("98").unwrap())
                .digits,
            vec![2, 2, 7]
        );
        assert_eq!(
            bigint("156483998155463") + bigint("15482265487796"),
            bigint("171966263643259")
        );
        assert_eq!(bigint("1298975") + bigint("6665"), bigint("1305640"));
        assert_eq!(bigint("1298975") + bigint("6665"), bigint("1305640"));
        assert_eq!(bigint("752") - bigint("354"), bigint("398"));
        assert_eq!(
            bigint("340282366920938463463374607431768211456")
                + bigint("565784967567542754765764575735654656546546555"),
            bigint("565785307849909675704228039110262088314758011")
        );
        assert_eq!(bigint("340282366920938463463374607431768211456") - bigint("565784967567542754765764575735654656546546555"), bigint("-565784627285175833827301112361047224778335099"));
    }

    fn bigint(s: &str) -> Bigint {
        Bigint::from_str(s).unwrap()
    }

    #[test]
    fn test_basic() {
        assert_eq!(Bigint::new(), bigint("0"));
        assert!(Bigint::from_str("foobar").is_err());

        assert!(bigint("1").is_positive());
        assert!(bigint("-1").is_negative());

        assert_eq!(bigint("123") + bigint("456"), bigint("579"));
        assert_eq!(bigint("579") - bigint("456"), bigint("123"));

        assert!(bigint("123") > bigint("122"));
    }
}
