use super::domain_error::DomainError;
use super::fft::fourier_transform::{ComplexValue, PolynomialTransform, InverseTransform, FourierTransform};
use super::bignum::Bignum;
use std::ops::{Add, Sub, Mul};
use std::str::FromStr;
impl FromStr for Bignum {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Bignum, DomainError> {

        let o = s.strip_prefix('-');
        let sign = o.is_some();
        let s = o.unwrap_or(s).to_uppercase();

        let (s, radix) = match s.get(.. 2) {
            Some("0B") => (&s[2..] ,2u8),
            Some("0O") => (&s[2..], 8),
            Some("0X") => (&s[2..], 16),
            _          => (s.as_str(), 10), // Defaults to base 10
        };

        let value = s.chars().rev()
          .map(|c| c.to_digit(radix as u32))
          .collect::<Vec<_>>();
        if value.iter().any(|&x| x.is_none()) {
            return Err(DomainError::ParsingError{input: s.to_string()});
        }
        let value = value.iter()
          .map(|&x| (x.unwrap()) as u8).collect::<Vec<_>>();
        if sign {
            Bignum::new_negative(value, radix)
        } else {
            Bignum::new_positive(value, radix)
        }
    }
}
impl Into<String> for Bignum {
    fn into(self) -> String {
        let radix = self.get_radix() as u32;
        let value = self.map(false).map(|x| x as u32).collect::<Vec<_>>();

        let value = value.iter().rev()
          .map(|&x| char::from_digit(x, radix))
          .collect::<Vec<Option<char>>>();

        let value = value.iter()
          .map(|&x| if let Some(x) = x { x } else { '0' });
        let mut prefix:Vec<String> = Vec::new();
        if self.has_sign() {prefix.push("-".to_string());};
        match radix {
            2 => prefix.push("0B".to_string()),
            8 => prefix.push("00".to_string()),
            16 => prefix.push("0X".to_string()),
            _ => ()
        }
        prefix.push(String::from_iter(value).to_uppercase());
        prefix.join("")
    }
}

impl<'a> Mul for &'a Bignum {
    type Output = Result<Bignum, DomainError>;

    fn mul(self, rhs: Self) -> Self::Output {
        let radix = self.get_radix();
        if radix != rhs.get_radix() {
            return Err(DomainError::NotSameRadix(radix, rhs.get_radix()))
        }
        let poly_trans = PolynomialTransform::new(8);
        let a = self.map(true).map(|x| ComplexValue::new(x as f64, 0.0)).collect::<Vec<_>>();
        let a = poly_trans.transform(&a);
        let b = rhs.map(true).map(|x| ComplexValue::new(x as f64, 0.0)).collect::<Vec<_>>();
        let b = poly_trans.transform(&b);
        let c = a.iter().zip(b.iter()).map(|(&x, &y)| x * y).collect::<Vec<_>>();
        let inverse_trans = InverseTransform::new(8);
        let result = inverse_trans.transform(&c);
        let result = result.iter().map(|&x| x.re.round() as i64);
        Bignum::from_iter(result, radix)
    }
}

impl<'a> Add for &'a Bignum {
    type Output = Result<Bignum, DomainError>;
    fn add(self, rhs: Self) -> Self::Output {
        let radix = self.get_radix();
        if radix != rhs.get_radix() {
            return Err(DomainError::NotSameRadix(radix, rhs.get_radix()))
        }
        let a = self.map(true);
        let b = rhs.map(true);
        let max_len = std::cmp::max(self.len(), rhs.len());
        let a = a.chain(std::iter::repeat(0));
        let b = b.chain(std::iter::repeat(0));
        let c= a.zip(b).take(max_len)
          .map(|(a_digit, b_digit)| a_digit + b_digit);
        Bignum::from_iter(c, radix)
    }
}

impl<'a> Sub for &'a Bignum {
    type Output = Result<Bignum, DomainError>;
    fn sub(self, rhs: Self) -> Self::Output {
       let rhs = rhs.reverse();
        self.add(&rhs)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{scenario, given, when, then};

    scenario!(bignum_from_string "test bignum from string" {
        {
            given!("a string with -0x prefix" {
                let num = "-0x12AFBCDE0";
            });
            when!("convert it to bignum" {
                let num = Bignum::from_str(num);
                assert!(num.is_ok());
                let num = num.unwrap();
            });
            then!("it should convert it to negative hex number" {
                assert!(num.has_sign());
                assert_eq!(num.get_radix(), 16);
            });
        }
        {
            given!("a string with 0B prefix" {
                let num = "0B111001";
            });
            when!("convert it to bignum" {
                let num = Bignum::from_str(num);
                assert!(num.is_ok());
                let num = num.unwrap();
            });
            then!("it should convert it binary positive number" {
                assert!(!num.has_sign());
                assert_eq!(num.get_radix(), 2);
            });
        }
        {
            given!("a string with hex digit but with 10 as radix by default" {
                let num = "12AFB0";
            });
            when!("convert it to bignum" {
                let num = Bignum::from_str(num);
            });
            then!("it should fail" {
                assert!(num.is_err());
            });
        }
        {
            given!("a string with -0o prefix" {
                let num = "-0o1276340";
            });
            when!("convert it to bignum" {
                let num = Bignum::from_str(num);
                assert!(num.is_ok());
                let num = num.unwrap();
            });
            then!("it should convert it oct negative number" {
                assert!(num.has_sign());
                assert_eq!(num.get_radix(), 8);
            });
        }
    });

    scenario!(bignum_to_string "test converting bignum to string" {
        {
            given!("negative number" {
                let num = Bignum::new_negative(vec![0, 1, 2, 5, 9], 10);
            });
            when!("cast it to string" {
                let num : String= num.unwrap().into();
            });
            then!("the output should have - prefix" {
                assert_eq!(num, "-95210");
            });
        }
        {
            given!("binary radix big number" {
                let num = Bignum::new_positive(vec![0, 1, 1, 1, 0], 2);
            });
            when!("cast it to string" {
                let num : String= num.unwrap().into();
            });
            then!("the output should have 0B prefix" {
                assert_eq!(num, "0B1110");
            });
        }

        {
            given!("hex negative number" {
                let num = Bignum::new_negative(vec![3, 10, 12, 15], 16);
            });
            when!("cast it to string" {
                let num : String= num.unwrap().into();
            });
            then!("the output should have -OX prefix" {
                assert_eq!(num, "-0XFCA3");
            });
        }
    });

    scenario!(bignum_equality "test bignum equality" {
        given!("hex big number with prefix zeros" {
            let num1 = Bignum::from_str("0X001230").unwrap();
        });
        {
            given!("another big number with same radix without prefix 0" {
                let num2 = Bignum::from_str("0X1230").unwrap();
            });
            then!("they are equal" {
                assert_eq!(num1, num2);
            });
        }
        {
            given!("another bignum with different radix" {
                let num3 = Bignum::from_str("0123").unwrap();
            });
            then!("they are not equal" {
                assert_ne!(num1, num3);
            });
        }
    });

    scenario!(bignum_addition "test bignum addition" {
        given!("two big number" {
            let num1 = &Bignum::from_str("12345678901234567890").unwrap();
            let num2 = &Bignum::from_str("98765432109876543210").unwrap();
        });
        when!("add both numbers" {
            let result = num1 + num2;
        });
        then!("the result should be the sum" {
            let expected = Bignum::from_str("111111111011111111100").unwrap();
            assert_eq!(result.unwrap(), expected);
        });
    });

    scenario!(bignum_subtraction "test bignum subtraction" {
        given!("two big number" {
            let num1 = &Bignum::from_str("12345678901234567890").unwrap();
            let num2 = &Bignum::from_str("98765432109876543210").unwrap();
        });
        when!("first one subtract the second" {
            let result = num1 - num2;
        });
        then!("the result should be the substraction" {
            let expected = Bignum::from_str("-86419753208641975320").unwrap();
            assert_eq!(result.unwrap(), expected);
        });
    });

    scenario!(decimal_bignum_multiplication
        "test decimal bignum multiplication" {
            given!("two big decimal number" {
            let num1 = &Bignum::from_str("-37").unwrap();
            let num2 = &Bignum::from_str("-176").unwrap();
            }
        );
        when!("multiply them" {
            let result = num1 * num2;
        });
        then!("the result should be the multiplication" {
            let expected = Bignum::from_str("6512").unwrap();
            assert_eq!(result.unwrap(), expected);
        });
    });

    scenario!(hex_bignum_multiplication "test hex bignum multiplication" {
        given!("two big number" {
            let num1 = &Bignum::from_str("0X2B5").unwrap();
            let num2 = &Bignum::from_str("-0XA4F").unwrap();
            }
        );
        when!("multiply them" {
            let result = num1 * num2;
        });
        then!("the result should be the multiplication" {
            let expected = Bignum::from_str("-0X1BE7DB").unwrap();
            assert_eq!(result.unwrap(), expected);
        });
    });

}

