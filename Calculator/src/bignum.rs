use std::iter::FromIterator;
use super::domain_error::DomainError;

pub type RadixType = u8;
#[derive(Debug, Clone)]
pub struct Bignum {
    value: Vec<RadixType>,
    radix: RadixType,
    sign: bool,
}

impl Bignum {
    pub fn new_positive(value: Vec<RadixType>, radix: RadixType)
        -> Result<Bignum, DomainError> {
        Self::new(value, radix, false)
    }
    pub fn new_negative(value: Vec<RadixType>, radix: RadixType)
        -> Result<Bignum, DomainError>
    {
        Self::new(value, radix,true)
    }
    pub fn from_iter(num: impl Iterator<Item=i64>, radix: RadixType)
                         -> Result<Bignum, DomainError> {
        let mut sign_num = 0;
        let mut to_higher = 0;
        let radix = radix as i64;
        let mut value: Vec<_> = Vec::with_capacity(num.size_hint().0 + 1);
        for digit in num {
            let mut digit = digit + to_higher;
            to_higher = digit / radix;
            digit %= radix;
            if digit != 0 {
                sign_num = digit.signum();
            }
            value.push(digit);
        }

        while to_higher != 0 {
            value.push(to_higher % radix);
            to_higher /= radix;
        }

        for digit in value.iter_mut() {
            let mut d = *digit * sign_num + to_higher;
            if d < 0 {
                d += radix;
                to_higher = -1;
            } else {
                to_higher = 0
            }
            *digit = d;
        }
        let radix = radix as u8;
        let value = value.into_iter()
          .map(|digit| digit as u8).collect();
        Self::new(value, radix, sign_num < 0)
    }

    fn new(value: Vec<RadixType>, radix: RadixType, sign: bool) -> Result<Bignum, DomainError> {
        if value.iter().all(|&x| x < radix) {
            if let Some(end) = value.iter()
              .rposition(|&x| x != 0) {
                let mut value = value;
                value.truncate(end + 1);
                Ok(Self{value, radix, sign})
            } else {
                Ok(Self{value: vec![0], radix, sign: false})
            }
        } else {
            Err(DomainError::RadixNotMatch(radix))
        }
    }
    pub fn len(&self) -> usize {
        self.value.len()
    }
    pub fn get_radix(&self) -> RadixType {self.radix}
    pub fn has_sign(&self) -> bool {self.sign}
    pub fn reverse(&self) -> Self {
        Self{value: self.value.clone(), radix: self.radix, sign: !self.sign}
    }

    pub fn map<'a>(&'a self, with_sign: bool)->impl Iterator<Item=i64>+'a {
        let sign = if self.sign && with_sign {-1} else {1};
        self.value.iter().map(move |&x| sign * (x as i64))
    }
}

impl From<Vec<RadixType>> for Bignum {
    fn from(values: Vec<RadixType>) -> Self {
        Self{
            radix: 16, sign: true, value: values
        }
    }
}

impl FromIterator<RadixType> for Bignum {
    fn from_iter<T: IntoIterator<Item=RadixType>>(iter: T) -> Self {
        let mut value = Vec::new();
        for i in iter {
            value.push(i);
        }
        Self{
            radix: 16, sign: true, value
        }
    }
}

impl PartialEq for Bignum {
    fn eq(&self, other: &Self) -> bool {
        self.radix == other.radix
          && self.sign == other.sign
        && self.value == other.value
    }
}

mod test {
    use super::*;
    use crate::{scenario, given, when, then};

    scenario!(bignum_new_with_given_radix "test new bignum with given radix" {
        {
            given!("invalid digits for radix 2" {
                let num = Bignum::new_positive(vec![0, 2, 3], 2);
            });
            then!("they should return errors" {
                assert!(num.is_err());
            });
        }
        {
            given!("invalid digits for radix 10" {
                let num = Bignum::new_negative(vec![0, 12, 3, 0], 10);
            });
            then!("they should return errors" {
                assert!(num.is_err());
            });
        }
    });

    scenario!(bignum_from_irregular_source "test new bignum from irregular source" {
        given!("an iterator with mixed signed digits" {
            let source = vec![0i64, -1, 12, -25, 1];
        });
        when!("constructing bignum from the iterator with radix 10" {
            let num = Bignum::from_iter(source.into_iter(), 10).unwrap();
        });
        then!("the result should be negative and normalized" {
            assert!(num.has_sign());
            let digits = num.map(false).collect::<Vec<_>>();
            assert_eq!(digits, vec![0, 1, 8, 3, 1]);
        });
    });

    scenario!(bignum_zero_without_sign "zero value should not have sign" {
        when!("creating a negative bignum that represents zero" {
            let num = Bignum::new_negative(vec![0, 0, 0], 2).unwrap();
        });
        then!("the zero should not carry a sign" {
            assert!(!num.has_sign());
        });
    });

    scenario!(bignum_map_with_sign "map exposes signed digits when requested" {
        given!("a negative bignum with trailing zeros" {
            let num = Bignum::new_negative(vec![0, 1, 0, 1, 0, 0], 2).unwrap();
        });
        when!("mapping with sign enabled" {
            let mapped = num.map(true).collect::<Vec<i64>>();
        });
        then!("the mapped digits should include negative signs and be trimmed" {
            assert!(num.has_sign());
            assert_eq!(mapped, vec![0, -1, 0, -1]);
        });
    });

    scenario!(bignum_reverse_sign "reverse flips sign without changing digits" {
        given!("a positive bignum" {
            let num_pos = Bignum::new_positive(vec![0, 3, 5, 2], 10).unwrap();
        });
        when!("reversing its sign" {
            let num_neg = num_pos.reverse();
        });
        then!("the sign should be flipped and digits preserved" {
            assert!(!num_pos.has_sign());
            assert!(num_neg.has_sign());
            let digits = num_neg.map(false).collect::<Vec<i64>>();
            assert_eq!(digits, vec![0, 3, 5, 2]);
        });
    });

    scenario!(bignum_equality "test bignum equality and inequality" {
        {
            given!("a negative hex number with prefix zeros" {
                let c1 = Bignum::new_negative(vec![0, 2, 3, 0], 10).unwrap();
            });
            then!("it will trim zero" {
                let c1_trim = Bignum::new_negative(vec![0, 2, 3, 0, 0], 10).unwrap();
                assert_eq!(c1, c1_trim);
            });
        }

        {
            given!("two numbers that differ by sign" {
                let p = Bignum::new_positive(vec![0, 2, 3, 0], 10).unwrap();
                let n = Bignum::new_negative(vec![0, 2, 3, 0, 0], 10).unwrap();
            });
            then!("they should not be equal" {
                assert_ne!(p, n);
            });
        }

        {
            given!("numbers that differ only by radix" {
                let n = Bignum::new_negative(vec![0, 2, 3, 0, 0], 10).unwrap();
                let r = Bignum::new_positive(vec![0, 2, 3, 0, 0], 16).unwrap();
            });
            then!("they should not be equal" {
                assert_ne!(n, r);
            });
        }
    });

}

