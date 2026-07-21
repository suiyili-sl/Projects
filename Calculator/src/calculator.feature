Feature: Big number arithmetic
  Scenario: Two big numbers addition
    Given two big numbers 0x1A2B3C4D and 0x5E6F7A8B
    When add them
    Then I should get 0x789AB6D8

  Scenario: Two big numbers multiplication
    Given two big numbers 0x1A2B3 and 0x5E6
    When multiply them
    Then The multiplication result should be 0x9A5ABD2