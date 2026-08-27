use std::str::FromStr;
use rmcp::{tool, tool_router};
use rmcp::handler::server::wrapper::Parameters;
use super::bignum::{Bignum};
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
  #[tool(description = "Add two big numbers")]
  fn add(&self, Parameters(Operands { a, b }): Parameters<Operands>) -> Result<String, String> {
    let r = Self::calculate(&a, &b, |a, b| a + b);
    r.map(|n| n.into()).map_err(|e| e.to_string())
  }

  #[tool(description = "Subtract two numbers")]
  fn subtract(&self, Parameters(Operands { a, b }): Parameters<Operands>) -> Result<String, String> {
    let r = Self::calculate(&a, &b, |a, b| a - b);
    r.map(|n| n.into()).map_err(|e| e.to_string())
  }

  #[tool(description = "Multiply two big numbers")]
  fn multiply(&self, Parameters(Operands { a, b }): Parameters<Operands>) -> Result<String, String> {
    let r = Self::calculate(&a, &b, |a, b| a * b);
    r.map(|n| n.into()).map_err(|e| e.to_string())
  }

  #[tool(description = "Divide big numbers")]
  fn divide(&self, Parameters(Operands { a, b }): Parameters<Operands>) -> Result<String, String> {
    todo!()
  }

  fn calculate<Calc>(a: &str, b: &str, f : Calc) -> Result<Bignum, DomainError>
  where Calc: Fn(&Bignum, &Bignum)->Result<Bignum, DomainError>
  {
    let a = Bignum::from_str(&a);
    let b = Bignum::from_str(&b);
    match (a, b){
      (Ok(a), Ok(b)) => {
        f(&a, &b)
      }
      _ => Err(DomainError::ParsingError {
        input: "number format is wrong!".to_string()
      })
    }
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
    result
  }

  #[then("I should get {expected:String}")]
  fn check_add_result(operands: &mut Operands, expected: String) -> StepResult<(), String> {
    let calculator = Calculator;
    let result = calculator.add(Parameters(Operands { a: operands.a.clone(), b: operands.b.clone() }));
    result.and_then(|n| if expected.eq_ignore_ascii_case(&n) {
      Ok(())
    } else {
      Err(format!("Expected: {}, but got: {}", expected, n))
    })
  }

  #[scenario("src/calculator.feature", name = "Two big numbers multiplication")]
  fn test_multiplication(operands: Operands) {

  }

  #[when("multiply them")]
  fn multiply_two_big_numbers(operands: &mut Operands) -> StepResult<String, String> {
    let calculator = Calculator;
    let result = calculator.multiply(Parameters(Operands { a: operands.a.clone(), b: operands.b.clone() }));
    result
  }

  #[then("The multiplication result should be {expected:String}")]
  fn check_multiply_result(operands: &mut Operands, expected: String) -> StepResult<(), String> {
    let calculator = Calculator;
    let result = calculator.multiply(Parameters(Operands { a: operands.a.clone(), b: operands.b.clone() }));
    result.and_then(|n| if expected.eq_ignore_ascii_case(&n) {
      Ok(())
    } else {
      Err(format!("Expected: {}, but got: {}", expected, n))
    })
  }

  #[scenario("src/calculator.feature", name = "Two big numbers subtract")]
  fn test_subtract(operands: Operands)
  {
  }

  #[when("subtract them")]
  fn subtract_two_big_numbers(operands: &mut Operands) -> StepResult<String, String>
  {
    let calculator = Calculator;
    let result = calculator.subtract(Parameters(Operands { a: operands.a.clone(), b: operands.b.clone() }));
    result
  }

  #[then("The subtract result should be {expected:String}")]
  fn check_subtract_result(operands: &mut Operands, expected: String)
    -> StepResult<(), String> {
    let calculator = Calculator;
    let result = calculator.subtract(Parameters(Operands { a: operands.a.clone(), b: operands.b.clone() }));
    result.and_then(|n| if expected.eq_ignore_ascii_case(&n) {
      Ok(())
    } else {
      Err(format!("Expected: {}, but got: {}", expected, n))
    })
  }

  #[scenario("src/calculator.feature", name = "Two big numbers division")]
  fn test_division(operands: Operands)
  {
  }

  #[when("one divide another")]
  fn divide_two_big_numbers(operands: &mut Operands) -> StepResult<String, String>
  {
    let calculator = Calculator;
    let result = calculator.divide(Parameters(Operands { a: operands.a.clone(), b: operands.b.clone() }));
    result
  }

  #[then("The division result should be {quotient:String} and {reminder:String}")]
  fn check_division_result(operands: &mut Operands, quotient: String, reminder: String)
                           -> StepResult<(), String> {
   todo!()
  }
}
