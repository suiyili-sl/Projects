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
  fn add(&self, Parameters(Operands { a, b }): Parameters<Operands>) -> String {
    Self::calculate(&a, &b, |a, b| a + b)
  }

  #[tool(description = "Subtract two numbers")]
  fn subtract(&self, Parameters(Operands { a, b }): Parameters<Operands>) ->String{
    Self::calculate(&a, &b, |a, b| a - b)
  }

  #[tool(description = "Multiply two big numbers")]
  fn multiply(&self, Parameters(Operands { a, b }): Parameters<Operands>) -> String {
    Self::calculate(&a, &b, |a, b| a * b)
  }

  fn calculate<Calc>(a: &str, b: &str, f : Calc) -> String
  where Calc: Fn(&Bignum, &Bignum)->Result<Bignum, DomainError>
  {
    let a = Bignum::from_str(&a);
    let b = Bignum::from_str(&b);
    match (a, b){
      (Ok(a), Ok(b)) => {
        let c = f(&a, &b);
        match c {
          Ok(c) => c.into(),
          Err(e) => e.to_string(),
        }
      }
      _ => "number format is wrong!".to_string()
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

  #[scenario("src/calculator.feature", name = "Two big numbers subtract")]
  fn test_subtract(operands: Operands)
  {
  }

  #[when("subtract them")]
  fn subtract_two_big_numbers(operands: &mut Operands) -> StepResult<String, String>
  {
    let calculator = Calculator;
    let result = calculator.subtract(Parameters(Operands { a: operands.a.clone(), b: operands.b.clone() }));
    Ok(result)
  }

  #[then("The subtract result should be {expected:String}")]
  fn check_subtract_result(operands: &mut Operands, expected: String)
    -> StepResult<(), String> {
    let calculator = Calculator;
    let result = calculator.subtract(Parameters(Operands { a: operands.a.clone(), b: operands.b.clone() }));
    if result.eq_ignore_ascii_case(&expected) {
      Ok(())
    } else {
      Err(format!("Expected: {}, but got: {}", expected, result))
    }
  }
}
