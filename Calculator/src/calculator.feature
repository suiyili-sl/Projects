Feature: Big number arithmetic
  Scenario: Two big numbers addition
    Given two big numbers 0x1A2B3C4D and 0x5E6F7A8B
    When add them
    Then I should get 0x789AB6D8

  Scenario: Two big numbers subtract
    Given two big numbers 0x1A2B3C4D and 0x5E6F7A8B
    When subtract them
    Then The subtract result should be -0x44443E3E

  Scenario: Two big numbers multiplication
    Given two big numbers 0x1A2B3 and 0x5E6
    When multiply them
    Then The multiplication result should be 0x9A5ABD2

  Scenario: Two big numbers division
    Given two big numbers 0x1A2B3C0739FE and 0x5E6387
    When one divide another
    Then The division result should be 0x46F993 and 0x1BC479