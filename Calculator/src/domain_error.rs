use thiserror::Error;

#[derive(Error, PartialEq, Debug)]
pub enum DomainError {
  #[error("The given input has wrong format for radix({0})")]
  RadixNotMatch(u8),

  #[error("Wrong format input {input}")]
  ParsingError{input: String},

  #[error("Two given number radix ({0}{1}) not same")]
  NotSameRadix(u8, u8),

  #[error("The given denominator is zero and not valid for operation")]
  DivisionByZero,
}

