use super::domain_error::DomainError;
use super::fft::fourier_transform::{ComplexValue, PolynomialTransform, InverseTransform, FourierTransform};
use std::fmt::Display;
use std::ops::{Add, Mul};

#[derive(Debug)]
pub struct Bignum <const BASE: u8> {
    value: Vec<u8>,
}
impl<const BASE : u8> Bignum<BASE> {
    pub fn new(value: Vec<u8>) -> Result<Self, DomainError> {
        let check = value.iter().all(|&x| x < BASE);
        if check {
            let mut value = value;
            value.reverse();
            Ok(
                Self {
                value
            })
        } else {
            Err(DomainError::InvalidDigit(BASE))
        }
    }

    pub fn from_str(num: &str) -> Result<Self, DomainError> {
        let base = BASE as u32;
        let value = num.chars()
          .map(|c| c.to_digit(base))
          .collect::<Vec<Option<u32>>>();
        if value.iter().any(|&x| x.is_none()) {
            return Err(DomainError::InvalidDigit(BASE));
        }
        Self::new(value.iter().map(|&x| x.unwrap() as u8).collect())
    }

    fn from_vector(num: impl Iterator<Item = u16>) -> Self {
        let mut radix_vec = Vec::new();
        radix_vec.reserve(num.size_hint().0 + 1);
        let mut to_higher = 0u16;
        for digit in num {
            let digit = digit + to_higher;
            to_higher = digit / BASE as u16;
            radix_vec.push((digit % BASE as u16) as u8);
        }
        Self{
            value: radix_vec
        }
    }

    pub fn to_string(&self) -> Result<String, DomainError> {
        let trimmed = self.trim_zeros();
        let base = BASE as u32;
        let value = trimmed.iter().rev()
          .map(|&x| char::from_digit( x as u32, base))
          .collect::<Vec<Option<char>>>();
        if value.iter().any(|&x| x.is_none()) {
            return Err(DomainError::InvalidDigit(BASE));
        }
        Ok(String::from_iter(value.iter().map(|&x| x.unwrap())))
    }

    fn trim_zeros(&self) -> &[u8] {
        if let Some(pos) = self.value.iter().rposition(|&x| x != 0) {
            &self.value[..=pos]
        } else {
            &self.value[..1] // Slice was entirely zeros
        }
    }
}
impl<const BASE : u8> PartialEq for Bignum<BASE> {
    fn eq(&self, other: &Self) -> bool {
        self.trim_zeros() == other.trim_zeros()
    }
}

impl<const BASE : u8> Display for Bignum<BASE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let trimmed = self.trim_zeros();
        for &digit in trimmed.iter().rev() {
            write!(f, "{}", digit)?;
        }
        Ok(())
    }
}

impl<const BASE : u8> Mul for Bignum<BASE> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let poly_trans = PolynomialTransform::new(8);
        let a = poly_trans.transform(&self.value.iter().map(|&x| ComplexValue::new(x as f64, 0.0)).collect::<Vec<_>>());
        let b = poly_trans.transform(&rhs.value.iter().map(|&x| ComplexValue::new(x as f64, 0.0)).collect::<Vec<_>>());
        let c = a.iter().zip(b.iter()).map(|(&x, &y)| x * y).collect::<Vec<_>>();
        let inverse_trans = InverseTransform::new(8);
        let result = inverse_trans.transform(&c);
        let result = result.iter().map(|&x| x.re.round() as u16);
        Bignum::from_vector(result)
    }
}

impl<const BASE : u8> Add for Bignum<BASE> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let max_len = std::cmp::max(self.value.len(), rhs.value.len());
        let a = self.value.iter().chain(std::iter::repeat(&0));
        let b = rhs.value.iter().chain(std::iter::repeat(&0));
        let c= a.zip(b).take(max_len).map(|(&a_digit, &b_digit)| (a_digit + b_digit) as u16);
        Self::from_vector(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bignum_base() {
        let num = Bignum::<10>::new(vec![0, 1, 2, 10, 0, 0]);
        assert!(num.is_err());
    }
    #[test]
    fn test_bignum_equality() {
        let num1 = Bignum::<10>::new(vec![0, 0, 1, 2, 3, 0]).unwrap();
        let num2 = Bignum::<10>::new(vec![1, 2, 3, 0]).unwrap();
        let num3 = Bignum::<10>::new(vec![0, 1, 2]).unwrap();

        assert_eq!(num1, num2);
        assert_ne!(num1, num3);
    }
    #[test]
    fn test_bignum_from_decimal() {
        let num = "01234567890";
        let decimal_num = Bignum::<10>::from_str(num).unwrap();
        let string_num = decimal_num.to_string().unwrap();
        assert_eq!(string_num, num.strip_prefix("0").unwrap());
    }

    #[test]
    fn test_bignum_from_hex() {
        let num = "0123456789ABCDEF0";
        let hex_num = Bignum::<16>::from_str(num).unwrap();
        let string_num = hex_num.to_string().unwrap();
        assert_eq!(string_num, num.to_lowercase().strip_prefix("0").unwrap());
    }

    #[test]
    fn test_bignum_addition() {
        let num1 = Bignum::<10>::from_str("12345678901234567890").unwrap();
        let num2 = Bignum::<10>::from_str("98765432109876543210").unwrap();
        let result = num1 + num2;
        let expected = Bignum::<10>::from_str("11111111011111111100").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_decimal_bignum_multiplication() {
        let num1 = Bignum::<10>::from_str("37").unwrap();
        let num2 = Bignum::<10>::from_str("176").unwrap();
        let result = num1 * num2;
        let expected = Bignum::<10>::from_str("6512").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hex_bignum_multiplication() {
        let num1 = Bignum::<16>::from_str("2B5").unwrap();
        let num2 = Bignum::<16>::from_str("A4F").unwrap();
        let result = num1 * num2;
        let expected = Bignum::<16>::from_str("1BE7DB").unwrap();
        assert_eq!(result, expected);
    }
}