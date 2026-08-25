use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
  #[error("The given input has wrong format for radix({0})")]
  RadixNotMatch(u8),

  #[error("Wrong format input {input}")]
  ParsingError{input: String},

  #[error("Two given number radix ({0}{1}) not same")]
  NotSameRadix(u8, u8),
}

