#[macro_export]
macro_rules! given { ($desc:literal { $($body:tt)*}) => {
    println!("Given: {}", $desc);
    $($body)*
}; }
#[macro_export]
macro_rules! when  { ($desc:literal { $($body:tt)*}) => {
    println!("When: {}", $desc);
    $($body)*
}; }
#[macro_export]
macro_rules! then  { ($desc:literal { $($body:tt)*}) => {
    println!("Then: {}", $desc);
    $($body)*
}; }
#[macro_export]
macro_rules! scenario {
    // 4. Entry Point: The user calls this macro to define a complete root scenario
    ( $name:ident $desc:literal $body:block ) => {
        #[test]
        fn $name () {
            println!("Scenario: {}", $desc);
            $body
        }
    };
}
