use rmcp::{tool, tool_router};
use rmcp::handler::server::wrapper::Parameters;
use super::bignum::Bignum;
use super::domain_error::DomainError;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct Operands {
  a: String,
  b: String,
}

#[derive(Clone)]
pub struct Calculator;
#[tool_router(server_handler)]
impl Calculator {
  fn same_base(a: &str, b: &str) -> Option<u8> {
    let comp = String::from_iter(a.chars().zip(b.chars())
      .take_while(|(a, b)| a == b)
      .map(|c| c.0)).to_uppercase();

    match comp.get(0..2) {
      Some("0X") => Some(16),
      Some("0B") => Some(2),
      Some("0O") => Some(8),
      _          => Some(10), // Defaults to base 10
    }
  }

  #[tool(description = "Add two big numbers")]
  fn add(&self, Parameters(Operands { a, b }): Parameters<Operands>) -> String {
    let comp = Self::same_base(&a, &b);
    if comp.is_none() {
      return "Error: Different bases are not supported".to_string();
    }
    let base = comp.unwrap();
    let result = match base {
      2 => {
        Self::add_base::<2>(&a, &b)
      }
      8 => {
        Self::add_base::<8>(&a, &b)
      }
      10 => {
        Self::add_base::<10>(&a, &b)
      }
      16 => {
        Self::add_base::<16>(&a, &b)
      }
      _ => Err(DomainError::InvalidDigit(base))
  };
    result.unwrap_or_else(|_| "Error: Invalid input for base {base}".to_string())
  }
  #[tool(description = "Multiply two big numbers")]
  fn multiply(&self, Parameters(Operands { a, b }): Parameters<Operands>) -> String {
    let comp = Self::same_base(&a, &b);
    if comp.is_none() {
      return "Error: Different bases are not supported".to_string();
    }
    let base = comp.unwrap();
    let result = match base {
      2 => {
        Self::multiply_base::<2>(&a, &b)
      }
      8 => {
        Self::multiply_base::<8>(&a, &b)
      }
      10 => {
        Self::multiply_base::<10>(&a, &b)
      }
      16 => {
        Self::multiply_base::<16>(&a, &b)
      }
      _ => Err(DomainError::InvalidDigit(base))
  };
    result.unwrap_or_else(|_| "Error: Invalid input for base {base}".to_string())
  }

  fn multiply_base<const BASE: u8>(a: &String, b: &String)
    -> Result<String, DomainError> {

    let a = Self::strip_prefix::<BASE>(&a);
    let b = Self::strip_prefix::<BASE>(&b);

    let a = Bignum::<BASE>::from_str(&a)?;
    let b = Bignum::<BASE>::from_str(&b)?;
    let c = a * b;
    let c = c.to_string()?.to_uppercase();
    Ok(Self::insert_prefix::<BASE>(&c))
  }

  fn add_base<const BASE: u8>(a: &String, b: &String)
    -> Result<String, DomainError> {
    let a = Self::strip_prefix::<BASE>(&a);
    let b = Self::strip_prefix::<BASE>(&b);

    let a = Bignum::<BASE>::from_str(&a)?;
    let b = Bignum::<BASE>::from_str(&b)?;
    let c = a + b;
    let c = c.to_string()?.to_uppercase();
    Ok(Self::insert_prefix::<BASE>(&c))
  }

  fn get_prefix<const BASE: u8>() -> &'static str {
    match BASE {
      2 => {
        "0B"
      }
      8 => {
        "0O"
      }
      16 => {
        "0X"
      }
      _ => ""
    }
  }
  fn strip_prefix<const BASE: u8>(input: &str) -> String {
    let input = input.to_uppercase();
    let prefix = Self::get_prefix::<BASE>();
    input.strip_prefix(prefix).unwrap_or(&input).to_string()
  }

  fn insert_prefix<const BASE: u8>(input: &str) -> String {
    let prefix = Self::get_prefix::<BASE>();
    format!("{}{}", prefix, input)
  }

}

#[cfg(test)]
mod test {
  use super::*;
  use rstest::fixture;
  use rstest_bdd::StepResult;
  use rstest_bdd_macros::{scenario, given, when, then, DataTable, DataTableRow, ScenarioState};

  #[fixture]
  fn operands() -> Operands {
    Operands { a: String::new(), b: String::new() }
  }

  #[scenario("src/calculator.feature", name = "Two big numbers addition")]
  fn test_addition(operands: Operands) {
  }

  #[given("two big numbers {a:String} and {b:String}")]
  fn set_two_big_numbers(operands: &mut Operands, a: String, b: String) -> StepResult<(), String> {
    operands.a = a;
    operands.b = b;
    Ok(())
  }

  #[when("add them")]
  fn add_two_big_numbers(operands: &mut Operands) -> StepResult<String, String> {
    let calculator = Calculator;
    let result = calculator.add(Parameters(Operands { a: operands.a.clone(), b: operands.b.clone() }));
    Ok(result)
  }

  #[then("I should get {expected:String}")]
  fn check_add_result(operands: &mut Operands, expected: String) -> StepResult<(), String> {
    let calculator = Calculator;
    let result = calculator.add(Parameters(Operands { a: operands.a.clone(), b: operands.b.clone() }));
    if result.eq_ignore_ascii_case(&expected) {
      Ok(())
    } else {
      Err(format!("Expected: {}, but got: {}", expected, result))
    }
  }

  #[scenario("src/calculator.feature", name = "Two big numbers multiplication")]
  fn test_multiplication(operands: Operands) {

  }

  #[when("multiply them")]
  fn multiply_two_big_numbers(operands: &mut Operands) -> StepResult<String, String> {
    let calculator = Calculator;
    let result = calculator.multiply(Parameters(Operands { a: operands.a.clone(), b: operands.b.clone() }));
    Ok(result)
  }

  #[then("The multiplication result should be {expected:String}")]
  fn check_multiply_result(operands: &mut Operands, expected: String) -> StepResult<(), String> {
    let calculator = Calculator;
    let result = calculator.multiply(Parameters(Operands { a: operands.a.clone(), b: operands.b.clone() }));
    if result.eq_ignore_ascii_case(&expected) {
      Ok(())
    } else {
      Err(format!("Expected: {}, but got: {}", expected, result))
    }
  }
}
